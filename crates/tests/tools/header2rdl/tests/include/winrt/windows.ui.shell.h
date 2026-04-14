
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
#ifndef __windows2Eui2Eshell_h__
#define __windows2Eui2Eshell_h__
#ifndef __windows2Eui2Eshell_p_h__
#define __windows2Eui2Eshell_p_h__


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

#if !defined(WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION)

#if !defined(WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Core.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
#include "Windows.UI.StartScreen.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IAdaptiveCard;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard ABI::Windows::UI::Shell::IAdaptiveCard

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IAdaptiveCardBuilderStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics ABI::Windows::UI::Shell::IAdaptiveCardBuilderStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IFocusSession;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIFocusSession ABI::Windows::UI::Shell::IFocusSession

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IFocusSessionManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager ABI::Windows::UI::Shell::IFocusSessionManager

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IFocusSessionManagerStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics ABI::Windows::UI::Shell::IFocusSessionManagerStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface ISecurityAppManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager ABI::Windows::UI::Shell::ISecurityAppManager

#endif // ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IShareWindowCommandEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs ABI::Windows::UI::Shell::IShareWindowCommandEventArgs

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IShareWindowCommandSource;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource ABI::Windows::UI::Shell::IShareWindowCommandSource

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IShareWindowCommandSourceStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics ABI::Windows::UI::Shell::IShareWindowCommandSourceStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface ITaskbarManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager ABI::Windows::UI::Shell::ITaskbarManager

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface ITaskbarManager2;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2 ABI::Windows::UI::Shell::ITaskbarManager2

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface ITaskbarManagerDesktopAppSupportStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics ABI::Windows::UI::Shell::ITaskbarManagerDesktopAppSupportStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface ITaskbarManagerStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics ABI::Windows::UI::Shell::ITaskbarManagerStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTab;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTab ABI::Windows::UI::Shell::IWindowTab

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabCloseRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs ABI::Windows::UI::Shell::IWindowTabCloseRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabCollection;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection ABI::Windows::UI::Shell::IWindowTabCollection

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabGroup;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup ABI::Windows::UI::Shell::IWindowTabGroup

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabIcon;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon ABI::Windows::UI::Shell::IWindowTabIcon

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabIconStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics ABI::Windows::UI::Shell::IWindowTabIconStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager ABI::Windows::UI::Shell::IWindowTabManager

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabManagerStatics;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics ABI::Windows::UI::Shell::IWindowTabManagerStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabSwitchRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs ABI::Windows::UI::Shell::IWindowTabSwitchRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabTearOutRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs ABI::Windows::UI::Shell::IWindowTabTearOutRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IWindowTabThumbnailRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs ABI::Windows::UI::Shell::IWindowTabThumbnailRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__

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
            namespace Shell {
                class WindowTab;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CShell__CWindowTab_USE
#define DEF___FIIterator_1_Windows__CUI__CShell__CWindowTab_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cadda39d-4148-5cbe-870b-c1ffaa000f16"))
IIterator<ABI::Windows::UI::Shell::WindowTab*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTab*, ABI::Windows::UI::Shell::IWindowTab*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Shell.WindowTab>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Shell::WindowTab*> __FIIterator_1_Windows__CUI__CShell__CWindowTab_t;
#define __FIIterator_1_Windows__CUI__CShell__CWindowTab ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CShell__CWindowTab_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CShell__CWindowTab_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CShell__CWindowTab_USE
#define DEF___FIIterable_1_Windows__CUI__CShell__CWindowTab_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf5a6d72-cc50-51b6-b34d-fdd7e59d33eb"))
IIterable<ABI::Windows::UI::Shell::WindowTab*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTab*, ABI::Windows::UI::Shell::IWindowTab*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Shell.WindowTab>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Shell::WindowTab*> __FIIterable_1_Windows__CUI__CShell__CWindowTab_t;
#define __FIIterable_1_Windows__CUI__CShell__CWindowTab ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CShell__CWindowTab_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CShell__CWindowTab_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CShell__CWindowTab_USE
#define DEF___FIVectorView_1_Windows__CUI__CShell__CWindowTab_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0a0bbd2-9503-516f-b079-d2b3a1842c0f"))
IVectorView<ABI::Windows::UI::Shell::WindowTab*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTab*, ABI::Windows::UI::Shell::IWindowTab*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Shell.WindowTab>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Shell::WindowTab*> __FIVectorView_1_Windows__CUI__CShell__CWindowTab_t;
#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CShell__CWindowTab_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CShell__CWindowTab_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CShell__CWindowTab_USE
#define DEF___FIVector_1_Windows__CUI__CShell__CWindowTab_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d2ee7da1-079a-5571-ac1e-e0761baa92e7"))
IVector<ABI::Windows::UI::Shell::WindowTab*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTab*, ABI::Windows::UI::Shell::IWindowTab*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Shell.WindowTab>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Shell::WindowTab*> __FIVector_1_Windows__CUI__CShell__CWindowTab_t;
#define __FIVector_1_Windows__CUI__CShell__CWindowTab ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CShell__CWindowTab_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CShell__CWindowTab_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class FocusSessionManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88ebdc98-220d-59cd-9cef-a34a27c36f98"))
ITypedEventHandler<ABI::Windows::UI::Shell::FocusSessionManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::FocusSessionManager*, ABI::Windows::UI::Shell::IFocusSessionManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.FocusSessionManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::FocusSessionManager*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class ShareWindowCommandSource;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class ShareWindowCommandEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e42513f4-d11b-511d-aedc-9f44ee28df04"))
ITypedEventHandler<ABI::Windows::UI::Shell::ShareWindowCommandSource*, ABI::Windows::UI::Shell::ShareWindowCommandEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::ShareWindowCommandSource*, ABI::Windows::UI::Shell::IShareWindowCommandSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::ShareWindowCommandEventArgs*, ABI::Windows::UI::Shell::IShareWindowCommandEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.ShareWindowCommandSource, Windows.UI.Shell.ShareWindowCommandEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::ShareWindowCommandSource*, ABI::Windows::UI::Shell::ShareWindowCommandEventArgs*> __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabCloseRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("963aec04-63d6-5d5f-b18d-785ca3d6850b"))
ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabCloseRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::IWindowTabManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabCloseRequestedEventArgs*, ABI::Windows::UI::Shell::IWindowTabCloseRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.WindowTabManager, Windows.UI.Shell.WindowTabCloseRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabCloseRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabSwitchRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92487ba4-93a1-5258-83a0-8bc055e89a25"))
ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabSwitchRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::IWindowTabManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabSwitchRequestedEventArgs*, ABI::Windows::UI::Shell::IWindowTabSwitchRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.WindowTabManager, Windows.UI.Shell.WindowTabSwitchRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabSwitchRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabTearOutRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9771fcc2-90a8-5353-8a7f-fc1ce40923cb"))
ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabTearOutRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::IWindowTabManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabTearOutRequestedEventArgs*, ABI::Windows::UI::Shell::IWindowTabTearOutRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.WindowTabManager, Windows.UI.Shell.WindowTabTearOutRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabTearOutRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabThumbnailRequestedEventArgs;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f07b88f9-f897-5ce5-9d2a-54778447fbdb"))
ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabThumbnailRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::IWindowTabManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::WindowTabThumbnailRequestedEventArgs*, ABI::Windows::UI::Shell::IWindowTabThumbnailRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.WindowTabManager, Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::WindowTabManager*, ABI::Windows::UI::Shell::WindowTabThumbnailRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_USE */

#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class AppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef struct BitmapSize BitmapSize;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

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
        namespace UI {
            namespace StartScreen {
                class SecondaryTile;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTile;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile ABI::Windows::UI::StartScreen::ISecondaryTile

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                typedef enum SecurityAppKind : int SecurityAppKind;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                typedef enum SecurityAppState : int SecurityAppState;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                typedef enum SecurityAppSubstatus : int SecurityAppSubstatus;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                typedef enum ShareWindowCommand : int ShareWindowCommand;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class FocusSession;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class TaskbarManager;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabCollection;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabGroup;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                class WindowTabIcon;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Shell.SecurityAppKind
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                enum SecurityAppKind : int
                {
                    SecurityAppKind_WebProtection = 0,
                };
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.SecurityAppState
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                enum SecurityAppState : int
                {
                    SecurityAppState_Disabled = 0,
                    SecurityAppState_Enabled = 1,
                };
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.SecurityAppSubstatus
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                enum SecurityAppSubstatus : int
                {
                    SecurityAppSubstatus_Undetermined = 0,
                    SecurityAppSubstatus_NoActionNeeded = 1,
                    SecurityAppSubstatus_ActionRecommended = 2,
                    SecurityAppSubstatus_ActionNeeded = 3,
                };
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.ShareWindowCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                enum ShareWindowCommand : int
                {
                    ShareWindowCommand_None = 0,
                    ShareWindowCommand_StartSharing = 1,
                    ShareWindowCommand_StopSharing = 2,
                };
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IAdaptiveCard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IAdaptiveCard[] = L"Windows.UI.Shell.IAdaptiveCard";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("72d0568c-a274-41cd-82a8-989d40b9b05e")
                IAdaptiveCard : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ToJson(
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdaptiveCard = __uuidof(IAdaptiveCard);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIAdaptiveCard;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IAdaptiveCardBuilderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IAdaptiveCardBuilderStatics[] = L"Windows.UI.Shell.IAdaptiveCardBuilderStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e")
                IAdaptiveCardBuilderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAdaptiveCardFromJson(
                        HSTRING value,
                        ABI::Windows::UI::Shell::IAdaptiveCard** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdaptiveCardBuilderStatics = __uuidof(IAdaptiveCardBuilderStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IFocusSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSession[] = L"Windows.UI.Shell.IFocusSession";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("069fbab8-0e84-5f2f-8614-9b6544326277")
                IFocusSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE End(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusSession = __uuidof(IFocusSession);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.IFocusSessionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSessionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSessionManager[] = L"Windows.UI.Shell.IFocusSessionManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("e7ffbaa9-d8be-5dbf-bac6-49364842e37e")
                IFocusSessionManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsFocusActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSession(
                        HSTRING id,
                        ABI::Windows::UI::Shell::IFocusSession** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryStartFocusSession(
                        ABI::Windows::UI::Shell::IFocusSession** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryStartFocusSession2(
                        ABI::Windows::Foundation::DateTime endTime,
                        ABI::Windows::UI::Shell::IFocusSession** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeactivateFocus(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsFocusActiveChanged(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsFocusActiveChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusSessionManager = __uuidof(IFocusSessionManager);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSessionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.IFocusSessionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSessionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSessionManagerStatics[] = L"Windows.UI.Shell.IFocusSessionManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("834df764-cb9a-5d0a-aa9f-73df4f249395")
                IFocusSessionManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::UI::Shell::IFocusSessionManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusSessionManagerStatics = __uuidof(IFocusSessionManagerStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.ISecurityAppManager
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.SecurityAppManager
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ISecurityAppManager[] = L"Windows.UI.Shell.ISecurityAppManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("96ac500c-aed4-561d-bde8-953520343a2d")
                ISecurityAppManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Register(
                        ABI::Windows::UI::Shell::SecurityAppKind kind,
                        HSTRING displayName,
                        ABI::Windows::Foundation::IUriRuntimeClass* detailsUri,
                        boolean registerPerUser,
                        GUID* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Unregister(
                        ABI::Windows::UI::Shell::SecurityAppKind kind,
                        GUID guidRegistration
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateState(
                        ABI::Windows::UI::Shell::SecurityAppKind kind,
                        GUID guidRegistration,
                        ABI::Windows::UI::Shell::SecurityAppState state,
                        ABI::Windows::UI::Shell::SecurityAppSubstatus substatus,
                        ABI::Windows::Foundation::IUriRuntimeClass* detailsUri
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecurityAppManager = __uuidof(ISecurityAppManager);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CISecurityAppManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandEventArgs[] = L"Windows.UI.Shell.IShareWindowCommandEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("4578dc09-a523-5756-a995-e4feb991fff0")
                IShareWindowCommandEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowId(
                        ABI::Windows::UI::WindowId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Command(
                        ABI::Windows::UI::Shell::ShareWindowCommand* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Command(
                        ABI::Windows::UI::Shell::ShareWindowCommand value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareWindowCommandEventArgs = __uuidof(IShareWindowCommandEventArgs);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandSource[] = L"Windows.UI.Shell.IShareWindowCommandSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("cb3b7ae3-6b9c-561e-bccc-61e68e0abfef")
                IShareWindowCommandSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportCommandChanged(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CommandRequested(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CommandRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CommandInvoked(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CommandInvoked(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareWindowCommandSource = __uuidof(IShareWindowCommandSource);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandSourceStatics[] = L"Windows.UI.Shell.IShareWindowCommandSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("b0eb6656-9cac-517c-b6c7-8ef715084295")
                IShareWindowCommandSourceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Shell::IShareWindowCommandSource** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareWindowCommandSourceStatics = __uuidof(IShareWindowCommandSourceStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManager[] = L"Windows.UI.Shell.ITaskbarManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("87490a19-1ad9-49f4-b2e8-86738dc5ac40")
                ITaskbarManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPinningAllowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsCurrentAppPinnedAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsAppListEntryPinnedAsync(
                        ABI::Windows::ApplicationModel::Core::IAppListEntry* appListEntry,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPinCurrentAppAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPinAppListEntryAsync(
                        ABI::Windows::ApplicationModel::Core::IAppListEntry* appListEntry,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITaskbarManager = __uuidof(ITaskbarManager);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Shell.ITaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManager2[] = L"Windows.UI.Shell.ITaskbarManager2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("79f0a06e-7b02-4911-918c-dee0bbd20ba4")
                ITaskbarManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSecondaryTilePinnedAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPinSecondaryTileAsync(
                        ABI::Windows::UI::StartScreen::ISecondaryTile* secondaryTile,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryUnpinSecondaryTileAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITaskbarManager2 = __uuidof(ITaskbarManager2);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManagerDesktopAppSupportStatics[] = L"Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("cdfefd63-e879-4134-b9a7-8283f05f9480")
                ITaskbarManagerDesktopAppSupportStatics : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ITaskbarManagerDesktopAppSupportStatics = __uuidof(ITaskbarManagerDesktopAppSupportStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManagerStatics[] = L"Windows.UI.Shell.ITaskbarManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("db32ab74-de52-4fe6-b7b6-95ff9f8395df")
                ITaskbarManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::UI::Shell::ITaskbarManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITaskbarManagerStatics = __uuidof(ITaskbarManagerStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IWindowTab
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTab
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTab[] = L"Windows.UI.Shell.IWindowTab";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("551e776a-7928-4d60-bdd9-672b5a5758eb")
                IWindowTab : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::UI::Shell::IWindowTabIcon** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Icon(
                        ABI::Windows::UI::Shell::IWindowTabIcon* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TreatAsSecondaryTileId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TreatAsSecondaryTileId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        ABI::Windows::UI::Shell::IWindowTabGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Group(
                        ABI::Windows::UI::Shell::IWindowTabGroup* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportThumbnailAvailable(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTab = __uuidof(IWindowTab);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTab;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabCloseRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabCloseRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabCloseRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabCloseRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("477282e9-eec4-5882-9889-2dd64d0f9fb6")
                IWindowTabCloseRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tab(
                        ABI::Windows::UI::Shell::IWindowTab** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabCloseRequestedEventArgs = __uuidof(IWindowTabCloseRequestedEventArgs);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabCollection
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabCollection
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabCollection[] = L"Windows.UI.Shell.IWindowTabCollection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("accd0d6c-ed07-519a-8c33-17e02e7e9b0f")
                IWindowTabCollection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE MoveTab(
                        ABI::Windows::UI::Shell::IWindowTab* tab,
                        UINT32 index
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabCollection = __uuidof(IWindowTabCollection);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabGroup
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabGroup
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabGroup[] = L"Windows.UI.Shell.IWindowTabGroup";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("a9c2c4fe-6cfe-449c-8b57-5756771abe56")
                IWindowTabGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::UI::Shell::IWindowTabIcon** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Icon(
                        ABI::Windows::UI::Shell::IWindowTabIcon* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabGroup = __uuidof(IWindowTabGroup);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabGroup;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabIcon
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabIcon
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabIcon[] = L"Windows.UI.Shell.IWindowTabIcon";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("f92f398f-3669-4d0c-a183-14ddae6f6538")
                IWindowTabIcon : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IWindowTabIcon = __uuidof(IWindowTabIcon);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabIcon;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabIconStatics
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabIcon
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabIconStatics[] = L"Windows.UI.Shell.IWindowTabIconStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("2e18d95e-2cbb-4084-af0c-36ee1c2d54b1")
                IWindowTabIconStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFontGlyph(
                        HSTRING glyph,
                        HSTRING fontFamily,
                        ABI::Windows::UI::Shell::IWindowTabIcon** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFontGlyphWithUri(
                        HSTRING glyph,
                        HSTRING fontFamily,
                        ABI::Windows::Foundation::IUriRuntimeClass* fontUri,
                        ABI::Windows::UI::Shell::IWindowTabIcon** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* image,
                        ABI::Windows::UI::Shell::IWindowTabIcon** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabIconStatics = __uuidof(IWindowTabIconStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabManager
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabManager
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabManager[] = L"Windows.UI.Shell.IWindowTabManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("97b3c697-f43a-43e7-b3a2-e889a9835599")
                IWindowTabManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tabs(
                        ABI::Windows::UI::Shell::IWindowTabCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetActiveTab(
                        ABI::Windows::UI::Shell::IWindowTab* tab
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TabSwitchRequested(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TabSwitchRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TabCloseRequested(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TabCloseRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TabTearOutRequested(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TabTearOutRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TabThumbnailRequested(
                        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TabThumbnailRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabManager = __uuidof(IWindowTabManager);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabManagerStatics
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabManager
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabManagerStatics[] = L"Windows.UI.Shell.IWindowTabManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("76755668-45f0-4e0b-8172-4e6d9d0f87bd")
                IWindowTabManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForWindow(
                        ABI::Windows::UI::WindowId id,
                        ABI::Windows::UI::Shell::IWindowTabManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsTabTearOutSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabManagerStatics = __uuidof(IWindowTabManagerStatics);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabSwitchRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabSwitchRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("7cbc421a-58a4-568b-a351-f8a947a5aad8")
                IWindowTabSwitchRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tab(
                        ABI::Windows::UI::Shell::IWindowTab** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabSwitchRequestedEventArgs = __uuidof(IWindowTabSwitchRequestedEventArgs);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabTearOutRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabTearOutRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("17d66659-5005-5ece-99af-566306e73642")
                IWindowTabTearOutRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tab(
                        ABI::Windows::UI::Shell::IWindowTab** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WindowId(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WindowId(
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabTearOutRequestedEventArgs = __uuidof(IWindowTabTearOutRequestedEventArgs);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabThumbnailRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                MIDL_INTERFACE("2d558e54-9c4e-5abc-ab72-3350fb4937a0")
                IWindowTabThumbnailRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tab(
                        ABI::Windows::UI::Shell::IWindowTab** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Image(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Image(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCompositedOnWindow(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowTabThumbnailRequestedEventArgs = __uuidof(IWindowTabThumbnailRequestedEventArgs);
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.AdaptiveCardBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IAdaptiveCardBuilderStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Shell_AdaptiveCardBuilder_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_AdaptiveCardBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_AdaptiveCardBuilder[] = L"Windows.UI.Shell.AdaptiveCardBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Shell.FocusSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IFocusSession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_FocusSession_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_FocusSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_FocusSession[] = L"Windows.UI.Shell.FocusSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.UI.Shell.FocusSessionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IFocusSessionManagerStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IFocusSessionManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_FocusSessionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_FocusSessionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_FocusSessionManager[] = L"Windows.UI.Shell.FocusSessionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.UI.Shell.SecurityAppManager
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.SecurityAppManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.ISecurityAppManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_SecurityAppManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_SecurityAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_SecurityAppManager[] = L"Windows.UI.Shell.SecurityAppManager";
#endif
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.ShareWindowCommandEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IShareWindowCommandEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_ShareWindowCommandEventArgs[] = L"Windows.UI.Shell.ShareWindowCommandEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Shell.ShareWindowCommandSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IShareWindowCommandSourceStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IShareWindowCommandSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_ShareWindowCommandSource[] = L"Windows.UI.Shell.ShareWindowCommandSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Shell.TaskbarManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Shell.ITaskbarManagerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.ITaskbarManager ** Default Interface **
 *    Windows.UI.Shell.ITaskbarManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Shell_TaskbarManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_TaskbarManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_TaskbarManager[] = L"Windows.UI.Shell.TaskbarManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Shell.WindowTab
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTab ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTab_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTab_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTab[] = L"Windows.UI.Shell.WindowTab";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabCloseRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabCloseRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabCloseRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabCloseRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabCloseRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabCloseRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabCollection
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabCollection ** Default Interface **
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Shell.WindowTab>
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Shell.WindowTab>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabCollection[] = L"Windows.UI.Shell.WindowTabCollection";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabGroup
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabGroup_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabGroup[] = L"Windows.UI.Shell.WindowTabGroup";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabIcon
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IWindowTabIconStatics interface starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabIcon ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabIcon_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabIcon_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabIcon[] = L"Windows.UI.Shell.WindowTabIcon";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabManager
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IWindowTabManagerStatics interface starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabManager[] = L"Windows.UI.Shell.WindowTabManager";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabSwitchRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabSwitchRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabTearOutRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabTearOutRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard;

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIFocusSession __x_ABI_CWindows_CUI_CShell_CIFocusSession;

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager;

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CISecurityAppManager __x_ABI_CWindows_CUI_CShell_CISecurityAppManager;

#endif // ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs;

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource;

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CITaskbarManager __x_ABI_CWindows_CUI_CShell_CITaskbarManager;

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CITaskbarManager2 __x_ABI_CWindows_CUI_CShell_CITaskbarManager2;

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTab __x_ABI_CWindows_CUI_CShell_CIWindowTab;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTab_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabManager __x_ABI_CWindows_CUI_CShell_CIWindowTabManager;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_FWD_DEFINED__

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

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CShell__CWindowTab __FIIterator_1_Windows__CUI__CShell__CWindowTab;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CShell__CWindowTab;

typedef struct __FIIterator_1_Windows__CUI__CShell__CWindowTabVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CShell__CWindowTabVtbl;

interface __FIIterator_1_Windows__CUI__CShell__CWindowTab
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CShell__CWindowTabVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CShell__CWindowTab_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CShell__CWindowTab __FIIterable_1_Windows__CUI__CShell__CWindowTab;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CShell__CWindowTab;

typedef struct __FIIterable_1_Windows__CUI__CShell__CWindowTabVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CShell__CWindowTab* This,
        __FIIterator_1_Windows__CUI__CShell__CWindowTab** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CShell__CWindowTabVtbl;

interface __FIIterable_1_Windows__CUI__CShell__CWindowTab
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CShell__CWindowTabVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CShell__CWindowTab_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CShell__CWindowTab __FIVectorView_1_Windows__CUI__CShell__CWindowTab;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CShell__CWindowTab;

typedef struct __FIVectorView_1_Windows__CUI__CShell__CWindowTabVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CShell__CWindowTabVtbl;

interface __FIVectorView_1_Windows__CUI__CShell__CWindowTab
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CShell__CWindowTabVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CShell__CWindowTab_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CShell__CWindowTab __FIVector_1_Windows__CUI__CShell__CWindowTab;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CShell__CWindowTab;

typedef struct __FIVector_1_Windows__CUI__CShell__CWindowTabVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        __FIVectorView_1_Windows__CUI__CShell__CWindowTab** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CShell__CWindowTab* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CShell__CWindowTabVtbl;

interface __FIVector_1_Windows__CUI__CShell__CWindowTab
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CShell__CWindowTabVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CShell__CWindowTab_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CShell__CWindowTab_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* This,
        __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* sender,
        __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabManager* sender,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabManager* sender,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabManager* sender,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabManager* sender,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CUI_CShell_CSecurityAppKind __x_ABI_CWindows_CUI_CShell_CSecurityAppKind;

typedef enum __x_ABI_CWindows_CUI_CShell_CSecurityAppState __x_ABI_CWindows_CUI_CShell_CSecurityAppState;

typedef enum __x_ABI_CWindows_CUI_CShell_CSecurityAppSubstatus __x_ABI_CWindows_CUI_CShell_CSecurityAppSubstatus;

typedef enum __x_ABI_CWindows_CUI_CShell_CShareWindowCommand __x_ABI_CWindows_CUI_CShell_CShareWindowCommand;

/*
 *
 * Struct Windows.UI.Shell.SecurityAppKind
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CShell_CSecurityAppKind
{
    SecurityAppKind_WebProtection = 0,
};
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.SecurityAppState
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CShell_CSecurityAppState
{
    SecurityAppState_Disabled = 0,
    SecurityAppState_Enabled = 1,
};
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.SecurityAppSubstatus
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CShell_CSecurityAppSubstatus
{
    SecurityAppSubstatus_Undetermined = 0,
    SecurityAppSubstatus_NoActionNeeded = 1,
    SecurityAppSubstatus_ActionRecommended = 2,
    SecurityAppSubstatus_ActionNeeded = 3,
};
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Shell.ShareWindowCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CUI_CShell_CShareWindowCommand
{
    ShareWindowCommand_None = 0,
    ShareWindowCommand_StartSharing = 1,
    ShareWindowCommand_StopSharing = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IAdaptiveCard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IAdaptiveCard[] = L"Windows.UI.Shell.IAdaptiveCard";
typedef struct __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ToJson)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_ToJson(This, result) \
    ((This)->lpVtbl->ToJson(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIAdaptiveCard;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IAdaptiveCardBuilderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IAdaptiveCardBuilderStatics[] = L"Windows.UI.Shell.IAdaptiveCardBuilderStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAdaptiveCardFromJson)(__x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics* This,
        HSTRING value,
        __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_CreateAdaptiveCardFromJson(This, value, result) \
    ((This)->lpVtbl->CreateAdaptiveCardFromJson(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIAdaptiveCardBuilderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IFocusSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSession[] = L"Windows.UI.Shell.IFocusSession";
typedef struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* End)(__x_ABI_CWindows_CUI_CShell_CIFocusSession* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIFocusSessionVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIFocusSession
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSession_End(This) \
    ((This)->lpVtbl->End(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.IFocusSessionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSessionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSessionManager[] = L"Windows.UI.Shell.IFocusSessionManager";
typedef struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsFocusActive)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetSession)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        HSTRING id,
        __x_ABI_CWindows_CUI_CShell_CIFocusSession** result);
    HRESULT (STDMETHODCALLTYPE* TryStartFocusSession)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        __x_ABI_CWindows_CUI_CShell_CIFocusSession** result);
    HRESULT (STDMETHODCALLTYPE* TryStartFocusSession2)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        __x_ABI_CWindows_CUI_CShell_CIFocusSession** result);
    HRESULT (STDMETHODCALLTYPE* DeactivateFocus)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This);
    HRESULT (STDMETHODCALLTYPE* add_IsFocusActiveChanged)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CFocusSessionManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsFocusActiveChanged)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_get_IsFocusActive(This, value) \
    ((This)->lpVtbl->get_IsFocusActive(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_GetSession(This, id, result) \
    ((This)->lpVtbl->GetSession(This, id, result))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_TryStartFocusSession(This, result) \
    ((This)->lpVtbl->TryStartFocusSession(This, result))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_TryStartFocusSession2(This, endTime, result) \
    ((This)->lpVtbl->TryStartFocusSession2(This, endTime, result))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_DeactivateFocus(This) \
    ((This)->lpVtbl->DeactivateFocus(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_add_IsFocusActiveChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsFocusActiveChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_remove_IsFocusActiveChanged(This, token) \
    ((This)->lpVtbl->remove_IsFocusActiveChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSessionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.IFocusSessionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.FocusSessionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IFocusSessionManagerStatics[] = L"Windows.UI.Shell.IFocusSessionManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        __x_ABI_CWindows_CUI_CShell_CIFocusSessionManager** result);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIFocusSessionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.ISecurityAppManager
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.SecurityAppManager
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ISecurityAppManager[] = L"Windows.UI.Shell.ISecurityAppManager";
typedef struct __x_ABI_CWindows_CUI_CShell_CISecurityAppManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Register)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        enum __x_ABI_CWindows_CUI_CShell_CSecurityAppKind kind,
        HSTRING displayName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* detailsUri,
        boolean registerPerUser,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* Unregister)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        enum __x_ABI_CWindows_CUI_CShell_CSecurityAppKind kind,
        GUID guidRegistration);
    HRESULT (STDMETHODCALLTYPE* UpdateState)(__x_ABI_CWindows_CUI_CShell_CISecurityAppManager* This,
        enum __x_ABI_CWindows_CUI_CShell_CSecurityAppKind kind,
        GUID guidRegistration,
        enum __x_ABI_CWindows_CUI_CShell_CSecurityAppState state,
        enum __x_ABI_CWindows_CUI_CShell_CSecurityAppSubstatus substatus,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* detailsUri);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CISecurityAppManagerVtbl;

interface __x_ABI_CWindows_CUI_CShell_CISecurityAppManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CISecurityAppManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_Register(This, kind, displayName, detailsUri, registerPerUser, result) \
    ((This)->lpVtbl->Register(This, kind, displayName, detailsUri, registerPerUser, result))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_Unregister(This, kind, guidRegistration) \
    ((This)->lpVtbl->Unregister(This, kind, guidRegistration))

#define __x_ABI_CWindows_CUI_CShell_CISecurityAppManager_UpdateState(This, kind, guidRegistration, state, substatus, detailsUri) \
    ((This)->lpVtbl->UpdateState(This, kind, guidRegistration, state, substatus, detailsUri))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CISecurityAppManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CISecurityAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandEventArgs[] = L"Windows.UI.Shell.IShareWindowCommandEventArgs";
typedef struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowId)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);
    HRESULT (STDMETHODCALLTYPE* get_Command)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        enum __x_ABI_CWindows_CUI_CShell_CShareWindowCommand* value);
    HRESULT (STDMETHODCALLTYPE* put_Command)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs* This,
        enum __x_ABI_CWindows_CUI_CShell_CShareWindowCommand value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_get_WindowId(This, value) \
    ((This)->lpVtbl->get_WindowId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_get_Command(This, value) \
    ((This)->lpVtbl->get_Command(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_put_Command(This, value) \
    ((This)->lpVtbl->put_Command(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandSource[] = L"Windows.UI.Shell.IShareWindowCommandSource";
typedef struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This);
    HRESULT (STDMETHODCALLTYPE* ReportCommandChanged)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This);
    HRESULT (STDMETHODCALLTYPE* add_CommandRequested)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CommandRequested)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CommandInvoked)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CShareWindowCommandSource_Windows__CUI__CShell__CShareWindowCommandEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CommandInvoked)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_ReportCommandChanged(This) \
    ((This)->lpVtbl->ReportCommandChanged(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_add_CommandRequested(This, handler, token) \
    ((This)->lpVtbl->add_CommandRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_remove_CommandRequested(This, token) \
    ((This)->lpVtbl->remove_CommandRequested(This, token))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_add_CommandInvoked(This, handler, token) \
    ((This)->lpVtbl->add_CommandInvoked(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_remove_CommandInvoked(This, token) \
    ((This)->lpVtbl->remove_CommandInvoked(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.IShareWindowCommandSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.ShareWindowCommandSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IShareWindowCommandSourceStatics[] = L"Windows.UI.Shell.IShareWindowCommandSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics* This,
        __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSource** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIShareWindowCommandSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManager[] = L"Windows.UI.Shell.ITaskbarManager";
typedef struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPinningAllowed)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsCurrentAppPinnedAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* IsAppListEntryPinnedAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* appListEntry,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestPinCurrentAppAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestPinAppListEntryAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* appListEntry,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CITaskbarManagerVtbl;

interface __x_ABI_CWindows_CUI_CShell_CITaskbarManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_get_IsPinningAllowed(This, value) \
    ((This)->lpVtbl->get_IsPinningAllowed(This, value))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_IsCurrentAppPinnedAsync(This, operation) \
    ((This)->lpVtbl->IsCurrentAppPinnedAsync(This, operation))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_IsAppListEntryPinnedAsync(This, appListEntry, operation) \
    ((This)->lpVtbl->IsAppListEntryPinnedAsync(This, appListEntry, operation))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_RequestPinCurrentAppAsync(This, operation) \
    ((This)->lpVtbl->RequestPinCurrentAppAsync(This, operation))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager_RequestPinAppListEntryAsync(This, appListEntry, operation) \
    ((This)->lpVtbl->RequestPinAppListEntryAsync(This, appListEntry, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Shell.ITaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManager2[] = L"Windows.UI.Shell.ITaskbarManager2";
typedef struct __x_ABI_CWindows_CUI_CShell_CITaskbarManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSecondaryTilePinnedAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        HSTRING tileId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestPinSecondaryTileAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* secondaryTile,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryUnpinSecondaryTileAsync)(__x_ABI_CWindows_CUI_CShell_CITaskbarManager2* This,
        HSTRING tileId,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CITaskbarManager2Vtbl;

interface __x_ABI_CWindows_CUI_CShell_CITaskbarManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CITaskbarManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_IsSecondaryTilePinnedAsync(This, tileId, operation) \
    ((This)->lpVtbl->IsSecondaryTilePinnedAsync(This, tileId, operation))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_RequestPinSecondaryTileAsync(This, secondaryTile, operation) \
    ((This)->lpVtbl->RequestPinSecondaryTileAsync(This, secondaryTile, operation))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManager2_TryUnpinSecondaryTileAsync(This, tileId, operation) \
    ((This)->lpVtbl->TryUnpinSecondaryTileAsync(This, tileId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManagerDesktopAppSupportStatics[] = L"Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerDesktopAppSupportStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Shell.ITaskbarManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.TaskbarManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_ITaskbarManagerStatics[] = L"Windows.UI.Shell.ITaskbarManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics* This,
        __x_ABI_CWindows_CUI_CShell_CITaskbarManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CITaskbarManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Shell.IWindowTab
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTab
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTab[] = L"Windows.UI.Shell.IWindowTab";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon** value);
    HRESULT (STDMETHODCALLTYPE* put_Icon)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* value);
    HRESULT (STDMETHODCALLTYPE* get_TreatAsSecondaryTileId)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_TreatAsSecondaryTileId)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup** value);
    HRESULT (STDMETHODCALLTYPE* put_Group)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* value);
    HRESULT (STDMETHODCALLTYPE* ReportThumbnailAvailable)(__x_ABI_CWindows_CUI_CShell_CIWindowTab* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTab
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_put_Icon(This, value) \
    ((This)->lpVtbl->put_Icon(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_get_TreatAsSecondaryTileId(This, value) \
    ((This)->lpVtbl->get_TreatAsSecondaryTileId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_put_TreatAsSecondaryTileId(This, value) \
    ((This)->lpVtbl->put_TreatAsSecondaryTileId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_put_Group(This, value) \
    ((This)->lpVtbl->put_Group(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTab_ReportThumbnailAvailable(This) \
    ((This)->lpVtbl->ReportThumbnailAvailable(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTab;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTab_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabCloseRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabCloseRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabCloseRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabCloseRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_get_Tab(This, value) \
    ((This)->lpVtbl->get_Tab(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCloseRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabCollection
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabCollection
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabCollection[] = L"Windows.UI.Shell.IWindowTabCollection";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MoveTab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabCollection* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* tab,
        UINT32 index);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabCollectionVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_MoveTab(This, tab, index) \
    ((This)->lpVtbl->MoveTab(This, tab, index))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabGroup
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabGroup
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabGroup[] = L"Windows.UI.Shell.IWindowTabGroup";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon** value);
    HRESULT (STDMETHODCALLTYPE* put_Icon)(__x_ABI_CWindows_CUI_CShell_CIWindowTabGroup* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabGroupVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_put_Icon(This, value) \
    ((This)->lpVtbl->put_Icon(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabGroup;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabIcon
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabIcon
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabIcon[] = L"Windows.UI.Shell.IWindowTabIcon";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabIconVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIcon* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabIconVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabIconVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabIcon;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIcon_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabIconStatics
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabIcon
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabIconStatics[] = L"Windows.UI.Shell.IWindowTabIconStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromFontGlyph)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        HSTRING glyph,
        HSTRING fontFamily,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromFontGlyphWithUri)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        HSTRING glyph,
        HSTRING fontFamily,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* fontUri,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromImage)(__x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* image,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabIcon** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_CreateFromFontGlyph(This, glyph, fontFamily, result) \
    ((This)->lpVtbl->CreateFromFontGlyph(This, glyph, fontFamily, result))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_CreateFromFontGlyphWithUri(This, glyph, fontFamily, fontUri, result) \
    ((This)->lpVtbl->CreateFromFontGlyphWithUri(This, glyph, fontFamily, fontUri, result))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_CreateFromImage(This, image, result) \
    ((This)->lpVtbl->CreateFromImage(This, image, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabIconStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabManager
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabManager
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabManager[] = L"Windows.UI.Shell.IWindowTabManager";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tabs)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabCollection** value);
    HRESULT (STDMETHODCALLTYPE* SetActiveTab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab* tab);
    HRESULT (STDMETHODCALLTYPE* add_TabSwitchRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabSwitchRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TabSwitchRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TabCloseRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabCloseRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TabCloseRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TabTearOutRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabTearOutRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TabTearOutRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TabThumbnailRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CWindowTabManager_Windows__CUI__CShell__CWindowTabThumbnailRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TabThumbnailRequested)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_get_Tabs(This, value) \
    ((This)->lpVtbl->get_Tabs(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_SetActiveTab(This, tab) \
    ((This)->lpVtbl->SetActiveTab(This, tab))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_add_TabSwitchRequested(This, handler, token) \
    ((This)->lpVtbl->add_TabSwitchRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_remove_TabSwitchRequested(This, token) \
    ((This)->lpVtbl->remove_TabSwitchRequested(This, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_add_TabCloseRequested(This, handler, token) \
    ((This)->lpVtbl->add_TabCloseRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_remove_TabCloseRequested(This, token) \
    ((This)->lpVtbl->remove_TabCloseRequested(This, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_add_TabTearOutRequested(This, handler, token) \
    ((This)->lpVtbl->add_TabTearOutRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_remove_TabTearOutRequested(This, token) \
    ((This)->lpVtbl->remove_TabTearOutRequested(This, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_add_TabThumbnailRequested(This, handler, token) \
    ((This)->lpVtbl->add_TabThumbnailRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManager_remove_TabThumbnailRequested(This, token) \
    ((This)->lpVtbl->remove_TabThumbnailRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabManagerStatics
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabManager
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabManagerStatics[] = L"Windows.UI.Shell.IWindowTabManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForWindow)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        struct __x_ABI_CWindows_CUI_CWindowId id,
        __x_ABI_CWindows_CUI_CShell_CIWindowTabManager** result);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsTabTearOutSupported)(__x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_GetForWindow(This, id, result) \
    ((This)->lpVtbl->GetForWindow(This, id, result))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_IsTabTearOutSupported(This, result) \
    ((This)->lpVtbl->IsTabTearOutSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabSwitchRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabSwitchRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_get_Tab(This, value) \
    ((This)->lpVtbl->get_Tab(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabSwitchRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabTearOutRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabTearOutRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** value);
    HRESULT (STDMETHODCALLTYPE* get_WindowId)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* put_WindowId)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_get_Tab(This, value) \
    ((This)->lpVtbl->get_Tab(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_get_WindowId(This, value) \
    ((This)->lpVtbl->get_WindowId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_put_WindowId(This, value) \
    ((This)->lpVtbl->put_WindowId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabTearOutRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_IWindowTabThumbnailRequestedEventArgs[] = L"Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tab)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIWindowTab** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedSize)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* value);
    HRESULT (STDMETHODCALLTYPE* get_Image)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_Image)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);
    HRESULT (STDMETHODCALLTYPE* get_IsCompositedOnWindow)(__x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_get_Tab(This, value) \
    ((This)->lpVtbl->get_Tab(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_get_RequestedSize(This, value) \
    ((This)->lpVtbl->get_RequestedSize(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_get_Image(This, value) \
    ((This)->lpVtbl->get_Image(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_put_Image(This, value) \
    ((This)->lpVtbl->put_Image(This, value))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#define __x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_get_IsCompositedOnWindow(This, value) \
    ((This)->lpVtbl->get_IsCompositedOnWindow(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CIWindowTabThumbnailRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.AdaptiveCardBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IAdaptiveCardBuilderStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Shell_AdaptiveCardBuilder_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_AdaptiveCardBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_AdaptiveCardBuilder[] = L"Windows.UI.Shell.AdaptiveCardBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Shell.FocusSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IFocusSession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_FocusSession_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_FocusSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_FocusSession[] = L"Windows.UI.Shell.FocusSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.UI.Shell.FocusSessionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IFocusSessionManagerStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IFocusSessionManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_FocusSessionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_FocusSessionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_FocusSessionManager[] = L"Windows.UI.Shell.FocusSessionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.UI.Shell.SecurityAppManager
 *
 * Introduced to Windows.UI.Shell.SecurityAppManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.SecurityAppManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.ISecurityAppManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_SecurityAppManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_SecurityAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_SecurityAppManager[] = L"Windows.UI.Shell.SecurityAppManager";
#endif
#endif // WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.ShareWindowCommandEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IShareWindowCommandEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_ShareWindowCommandEventArgs[] = L"Windows.UI.Shell.ShareWindowCommandEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Shell.ShareWindowCommandSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IShareWindowCommandSourceStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IShareWindowCommandSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_ShareWindowCommandSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_ShareWindowCommandSource[] = L"Windows.UI.Shell.ShareWindowCommandSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Shell.TaskbarManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.ITaskbarManagerDesktopAppSupportStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Shell.ITaskbarManagerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.ITaskbarManager ** Default Interface **
 *    Windows.UI.Shell.ITaskbarManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Shell_TaskbarManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_TaskbarManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_TaskbarManager[] = L"Windows.UI.Shell.TaskbarManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Shell.WindowTab
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTab ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTab_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTab_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTab[] = L"Windows.UI.Shell.WindowTab";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabCloseRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabCloseRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabCloseRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabCloseRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabCloseRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabCloseRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabCollection
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabCollection ** Default Interface **
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Shell.WindowTab>
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Shell.WindowTab>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabCollection[] = L"Windows.UI.Shell.WindowTabCollection";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabGroup
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabGroup_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabGroup[] = L"Windows.UI.Shell.WindowTabGroup";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabIcon
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IWindowTabIconStatics interface starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabIcon ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabIcon_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabIcon_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabIcon[] = L"Windows.UI.Shell.WindowTabIcon";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabManager
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.IWindowTabManagerStatics interface starting with version 1.0 of the Windows.UI.Shell.WindowTabManagerContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabManager[] = L"Windows.UI.Shell.WindowTabManager";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabSwitchRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabSwitchRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabSwitchRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabSwitchRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabTearOutRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabTearOutRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabTearOutRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabTearOutRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs
 *
 * Introduced to Windows.UI.Shell.WindowTabManagerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.IWindowTabThumbnailRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_WindowTabThumbnailRequestedEventArgs[] = L"Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs";
#endif
#endif // WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Eshell_p_h__

#endif // __windows2Eui2Eshell_h__
