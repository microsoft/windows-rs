
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
#ifndef __windows2Eui2Examl2Ehosting_h__
#define __windows2Eui2Examl2Ehosting_h__
#ifndef __windows2Eui2Examl2Ehosting_p_h__
#define __windows2Eui2Examl2Ehosting_p_h__


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

#if !defined(WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION)
#define WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.Composition.h"
#include "Windows.UI.WindowManagement.h"
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Controls.h"
#include "Windows.UI.Xaml.Controls.Primitives.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesignerAppExitedEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs ABI::Windows::UI::Xaml::Hosting::IDesignerAppExitedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesignerAppManager;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager ABI::Windows::UI::Xaml::Hosting::IDesignerAppManager

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesignerAppManagerFactory;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory ABI::Windows::UI::Xaml::Hosting::IDesignerAppManagerFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesignerAppView;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView ABI::Windows::UI::Xaml::Hosting::IDesignerAppView

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesktopWindowXamlSource;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesktopWindowXamlSourceFactory;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSourceFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesktopWindowXamlSourceGotFocusEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSourceGotFocusEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IDesktopWindowXamlSourceTakeFocusRequestedEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSourceTakeFocusRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IElementCompositionPreview;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview ABI::Windows::UI::Xaml::Hosting::IElementCompositionPreview

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IElementCompositionPreviewStatics;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics ABI::Windows::UI::Xaml::Hosting::IElementCompositionPreviewStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IElementCompositionPreviewStatics2;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2 ABI::Windows::UI::Xaml::Hosting::IElementCompositionPreviewStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IElementCompositionPreviewStatics3;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3 ABI::Windows::UI::Xaml::Hosting::IElementCompositionPreviewStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IWindowsXamlManager;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager ABI::Windows::UI::Xaml::Hosting::IWindowsXamlManager

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IWindowsXamlManagerStatics;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics ABI::Windows::UI::Xaml::Hosting::IWindowsXamlManagerStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlSourceFocusNavigationRequest;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlSourceFocusNavigationRequestFactory;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequestFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlSourceFocusNavigationResult;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationResult

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlSourceFocusNavigationResultFactory;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationResultFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenter;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenter

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenterHost;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterHost

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenterHost2;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2 ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterHost2

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenterHost3;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3 ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterHost3

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenterStatics;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    interface IXamlUIPresenterStatics2;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2 ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesignerAppView;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("49c6c658-1bd2-581e-a385-6eb3fd9bfee3"))
IAsyncOperation<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*, ABI::Windows::UI::Xaml::Hosting::IDesignerAppView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Xaml.Hosting.DesignerAppView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*> __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_t;
#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE */

#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aeeb272e-a814-5981-a2c3-623e226e4a71"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*, ABI::Windows::UI::Xaml::Hosting::IDesignerAppView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Xaml.Hosting.DesignerAppView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Xaml::Hosting::DesignerAppView*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_USE */

#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesignerAppManager;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesignerAppExitedEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3772ce98-9ba0-504b-8a0d-36b7f816a1c2"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesignerAppManager*, ABI::Windows::UI::Xaml::Hosting::DesignerAppExitedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesignerAppManager*, ABI::Windows::UI::Xaml::Hosting::IDesignerAppManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesignerAppExitedEventArgs*, ABI::Windows::UI::Xaml::Hosting::IDesignerAppExitedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Hosting.DesignerAppManager, Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesignerAppManager*, ABI::Windows::UI::Xaml::Hosting::DesignerAppExitedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_USE */

#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesktopWindowXamlSource;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesktopWindowXamlSourceGotFocusEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a77f0e84-ced7-5681-b140-8c18a55ceb1d"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceGotFocusEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceGotFocusEventArgs*, ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSourceGotFocusEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Hosting.DesktopWindowXamlSource, Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceGotFocusEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_USE */

#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class DesktopWindowXamlSourceTakeFocusRequestedEventArgs;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9e79eafe-c7fe-54a5-96ff-7abe02e05418"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceTakeFocusRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceTakeFocusRequestedEventArgs*, ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSourceTakeFocusRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Hosting.DesktopWindowXamlSource, Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource*, ABI::Windows::UI::Xaml::Hosting::DesktopWindowXamlSourceTakeFocusRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_USE */

#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

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
        namespace UI {
            namespace Composition {
                class CompositionPropertySet;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface ICompositionPropertySet;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet ABI::Windows::UI::Composition::ICompositionPropertySet

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface ICompositionAnimationBase;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase ABI::Windows::UI::Composition::ICompositionAnimationBase

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Controls {
                    namespace Primitives {
                        typedef enum FlyoutPlacementMode : int FlyoutPlacementMode;
                    } /* Primitives */
                } /* Controls */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Controls {
                    class ScrollViewer;
                } /* Controls */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Controls {
                    interface IScrollViewer;
                } /* Controls */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer ABI::Windows::UI::Xaml::Controls::IScrollViewer

#endif // ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class FrameworkElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IFrameworkElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement ABI::Windows::UI::Xaml::IFrameworkElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class ResourceDictionary;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IResourceDictionary;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIResourceDictionary ABI::Windows::UI::Xaml::IResourceDictionary

#endif // ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class UIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IUIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIUIElement ABI::Windows::UI::Xaml::IUIElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    typedef enum DesignerAppViewState : int DesignerAppViewState;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    typedef enum XamlSourceFocusNavigationReason : int XamlSourceFocusNavigationReason;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class WindowsXamlManager;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class XamlSourceFocusNavigationRequest;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    class XamlSourceFocusNavigationResult;
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Hosting.DesignerAppViewState
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    enum DesignerAppViewState : int
                    {
                        DesignerAppViewState_Visible = 0,
                        DesignerAppViewState_Hidden = 1,
                    };
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    enum XamlSourceFocusNavigationReason : int
                    {
                        XamlSourceFocusNavigationReason_Programmatic = 0,
                        XamlSourceFocusNavigationReason_Restore = 1,
                        XamlSourceFocusNavigationReason_First = 3,
                        XamlSourceFocusNavigationReason_Last = 4,
                        XamlSourceFocusNavigationReason_Left = 7,
                        XamlSourceFocusNavigationReason_Up = 8,
                        XamlSourceFocusNavigationReason_Right = 9,
                        XamlSourceFocusNavigationReason_Down = 10,
                    };
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppExitedEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("f6aac86a-0cad-410c-8f62-dc2936151c74")
                    IDesignerAppExitedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExitCode(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesignerAppExitedEventArgs = __uuidof(IDesignerAppExitedEventArgs);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppManager[] = L"Windows.UI.Xaml.Hosting.IDesignerAppManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("a6272caa-d5c6-40cb-abd9-27ba43831bb7")
                    IDesignerAppManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DesignerAppExited(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DesignerAppExited(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateNewViewAsync(
                            ABI::Windows::UI::Xaml::Hosting::DesignerAppViewState initialViewState,
                            ABI::Windows::Foundation::Size initialViewSize,
                            __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadObjectIntoAppAsync(
                            HSTRING dllName,
                            GUID classId,
                            HSTRING initializationData,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesignerAppManager = __uuidof(IDesignerAppManager);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppManagerFactory[] = L"Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("8f9d633b-1266-4c0e-8499-0db85bbd4c43")
                    IDesignerAppManagerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING appUserModelId,
                            ABI::Windows::UI::Xaml::Hosting::IDesignerAppManager** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesignerAppManagerFactory = __uuidof(IDesignerAppManagerFactory);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppView
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppView
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppView[] = L"Windows.UI.Xaml.Hosting.IDesignerAppView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("5c777cea-dd71-4a84-a56f-dacb4b14706f")
                    IDesignerAppView : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ApplicationViewId(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ViewState(
                            ABI::Windows::UI::Xaml::Hosting::DesignerAppViewState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ViewSize(
                            ABI::Windows::Foundation::Size* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateViewAsync(
                            ABI::Windows::UI::Xaml::Hosting::DesignerAppViewState viewState,
                            ABI::Windows::Foundation::Size viewSize,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesignerAppView = __uuidof(IDesignerAppView);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSource[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("d585bfe1-00ff-51be-ba1d-a1329956ea0a")
                    IDesktopWindowXamlSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Content(
                            ABI::Windows::UI::Xaml::IUIElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Content(
                            ABI::Windows::UI::Xaml::IUIElement* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasFocus(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_TakeFocusRequested(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_TakeFocusRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GotFocus(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GotFocus(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NavigateFocus(
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest* request,
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesktopWindowXamlSource = __uuidof(IDesktopWindowXamlSource);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceFactory[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("5cd61dc0-2561-56e1-8e75-6e44173805e3")
                    IDesktopWindowXamlSourceFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Hosting::IDesktopWindowXamlSource** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesktopWindowXamlSourceFactory = __uuidof(IDesktopWindowXamlSourceFactory);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceGotFocusEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("39be4849-d9cc-5b70-8f05-1ad9a4aaa342")
                    IDesktopWindowXamlSourceGotFocusEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesktopWindowXamlSourceGotFocusEventArgs = __uuidof(IDesktopWindowXamlSourceGotFocusEventArgs);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceTakeFocusRequestedEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2")
                    IDesktopWindowXamlSourceTakeFocusRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDesktopWindowXamlSourceTakeFocusRequestedEventArgs = __uuidof(IDesktopWindowXamlSourceTakeFocusRequestedEventArgs);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreview[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreview";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("b6f1a676-cfe6-46ac-acf6-c4687bb65e60")
                    IElementCompositionPreview : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IElementCompositionPreview = __uuidof(IElementCompositionPreview);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("08c92b38-ec99-4c55-bc85-a1c180b27646")
                    IElementCompositionPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetElementVisual(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Composition::IVisual** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetElementChildVisual(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Composition::IVisual** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetElementChildVisual(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Composition::IVisual* visual
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetScrollViewerManipulationPropertySet(
                            ABI::Windows::UI::Xaml::Controls::IScrollViewer* scrollViewer,
                            ABI::Windows::UI::Composition::ICompositionPropertySet** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IElementCompositionPreviewStatics = __uuidof(IElementCompositionPreviewStatics);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics2[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("24148fbb-23d6-4f37-ba0c-0733e799722d")
                    IElementCompositionPreviewStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetImplicitShowAnimation(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Composition::ICompositionAnimationBase* animation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetImplicitHideAnimation(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Composition::ICompositionAnimationBase* animation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetIsTranslationEnabled(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPointerPositionPropertySet(
                            ABI::Windows::UI::Xaml::IUIElement* targetElement,
                            ABI::Windows::UI::Composition::ICompositionPropertySet** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IElementCompositionPreviewStatics2 = __uuidof(IElementCompositionPreviewStatics2);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics3[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("843bc4c3-c105-59fe-a3d1-373c1d3e6fbc")
                    IElementCompositionPreviewStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetAppWindowContent(
                            ABI::Windows::UI::WindowManagement::IAppWindow* appWindow,
                            ABI::Windows::UI::Xaml::IUIElement* xamlContent
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAppWindowContent(
                            ABI::Windows::UI::WindowManagement::IAppWindow* appWindow,
                            ABI::Windows::UI::Xaml::IUIElement** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IElementCompositionPreviewStatics3 = __uuidof(IElementCompositionPreviewStatics3);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IWindowsXamlManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IWindowsXamlManager[] = L"Windows.UI.Xaml.Hosting.IWindowsXamlManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("56096c31-1aa0-5288-8818-6e74a2dcaff5")
                    IWindowsXamlManager : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IWindowsXamlManager = __uuidof(IWindowsXamlManager);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IWindowsXamlManagerStatics[] = L"Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("28258a12-7d82-505b-b210-712b04a58882")
                    IWindowsXamlManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE InitializeForCurrentThread(
                            ABI::Windows::UI::Xaml::Hosting::IWindowsXamlManager** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWindowsXamlManagerStatics = __uuidof(IWindowsXamlManagerStatics);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationRequest[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("fbb93bb5-1496-5a80-ac00-e757359755e6")
                    IXamlSourceFocusNavigationRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::UI::Xaml::Hosting::XamlSourceFocusNavigationReason* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HintRect(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                            GUID* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlSourceFocusNavigationRequest = __uuidof(IXamlSourceFocusNavigationRequest);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationRequestFactory[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("e746ab8f-b4ef-5390-97e5-cc0a2779c574")
                    IXamlSourceFocusNavigationRequestFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::UI::Xaml::Hosting::XamlSourceFocusNavigationReason reason,
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithHintRect(
                            ABI::Windows::UI::Xaml::Hosting::XamlSourceFocusNavigationReason reason,
                            ABI::Windows::Foundation::Rect hintRect,
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithHintRectAndCorrelationId(
                            ABI::Windows::UI::Xaml::Hosting::XamlSourceFocusNavigationReason reason,
                            ABI::Windows::Foundation::Rect hintRect,
                            GUID correlationId,
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlSourceFocusNavigationRequestFactory = __uuidof(IXamlSourceFocusNavigationRequestFactory);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationResult[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("88d55a5f-9603-5d8f-9cc7-d1c4070d9801")
                    IXamlSourceFocusNavigationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_WasFocusMoved(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlSourceFocusNavigationResult = __uuidof(IXamlSourceFocusNavigationResult);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationResultFactory[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("43bbadbf-f9e1-5527-b8c5-09339ff2ca76")
                    IXamlSourceFocusNavigationResultFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            boolean focusMoved,
                            ABI::Windows::UI::Xaml::Hosting::IXamlSourceFocusNavigationResult** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlSourceFocusNavigationResultFactory = __uuidof(IXamlSourceFocusNavigationResultFactory);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenter
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenter[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenter";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("a714944a-1619-4fc6-b31b-89512ef022a2")
                    IXamlUIPresenter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RootElement(
                            ABI::Windows::UI::Xaml::IUIElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RootElement(
                            ABI::Windows::UI::Xaml::IUIElement* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThemeKey(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ThemeKey(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThemeResourcesXaml(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ThemeResourcesXaml(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSize(
                            INT32 width,
                            INT32 height
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Render(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Present(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenter = __uuidof(IXamlUIPresenter);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659")
                    IXamlUIPresenterHost : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ResolveFileResource(
                            HSTRING path,
                            HSTRING* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenterHost = __uuidof(IXamlUIPresenterHost);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost2[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("61595672-7ca4-4a21-b56a-88f4812388ca")
                    IXamlUIPresenterHost2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetGenericXamlFilePath(
                            HSTRING* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenterHost2 = __uuidof(IXamlUIPresenterHost2);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost3[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("b14292bf-7320-41bb-9f26-4d6fd34db45a")
                    IXamlUIPresenterHost3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ResolveDictionaryResource(
                            ABI::Windows::UI::Xaml::IResourceDictionary* dictionary,
                            IInspectable* dictionaryKey,
                            IInspectable* suggestedValue,
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenterHost3 = __uuidof(IXamlUIPresenterHost3);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterStatics[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("71eaeac8-45e1-4192-85aa-3a422edd23cf")
                    IXamlUIPresenterStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CompleteTimelinesAutomatically(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CompleteTimelinesAutomatically(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHost(
                            ABI::Windows::UI::Xaml::Hosting::IXamlUIPresenterHost* host
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyWindowSizeChanged(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenterStatics = __uuidof(IXamlUIPresenterStatics);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterStatics2[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Hosting {
                    MIDL_INTERFACE("5c6b68d2-cf1c-4f53-bf09-6a745f7a9703")
                    IXamlUIPresenterStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetFlyoutPlacementTargetInfo(
                            ABI::Windows::UI::Xaml::IFrameworkElement* placementTarget,
                            ABI::Windows::UI::Xaml::Controls::Primitives::FlyoutPlacementMode preferredPlacement,
                            ABI::Windows::UI::Xaml::Controls::Primitives::FlyoutPlacementMode* targetPreferredPlacement,
                            boolean* allowFallbacks,
                            ABI::Windows::Foundation::Rect* returnValue
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFlyoutPlacement(
                            ABI::Windows::Foundation::Rect placementTargetBounds,
                            ABI::Windows::Foundation::Size controlSize,
                            ABI::Windows::Foundation::Size minControlSize,
                            ABI::Windows::Foundation::Rect containerRect,
                            ABI::Windows::UI::Xaml::Controls::Primitives::FlyoutPlacementMode targetPreferredPlacement,
                            boolean allowFallbacks,
                            ABI::Windows::UI::Xaml::Controls::Primitives::FlyoutPlacementMode* chosenPlacement,
                            ABI::Windows::Foundation::Rect* returnValue
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlUIPresenterStatics2 = __uuidof(IXamlUIPresenterStatics2);
                } /* Hosting */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs[] = L"Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory interface starting with version 2.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppManager ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppManager[] = L"Windows.UI.Xaml.Hosting.DesignerAppManager";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppView
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppView ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppView_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppView[] = L"Windows.UI.Xaml.Hosting.DesignerAppView";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IElementCompositionPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_ElementCompositionPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_ElementCompositionPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_ElementCompositionPreview[] = L"Windows.UI.Xaml.Hosting.ElementCompositionPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IWindowsXamlManager ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_WindowsXamlManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_WindowsXamlManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_WindowsXamlManager[] = L"Windows.UI.Xaml.Hosting.WindowsXamlManager";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest[] = L"Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult[] = L"Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics interface starting with version 1.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2 interface starting with version 1.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlUIPresenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlUIPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlUIPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlUIPresenter[] = L"Windows.UI.Xaml.Hosting.XamlUIPresenter";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2 __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3 __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2 __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3 __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2 __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView;

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView;

typedef struct __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* This,
        __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CHosting__CDesignerAppView_INTERFACE_DEFINED__
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* sender,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* sender,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* sender,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet __x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet;

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase __x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase;

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisual __x_ABI_CWindows_CUI_CComposition_CIVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer __x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer;

#endif // ____x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIResourceDictionary __x_ABI_CWindows_CUI_CXaml_CIResourceDictionary;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIResourceDictionary_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState;

typedef enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason;

/*
 *
 * Struct Windows.UI.Xaml.Hosting.DesignerAppViewState
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState
{
    DesignerAppViewState_Visible = 0,
    DesignerAppViewState_Hidden = 1,
};
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason
{
    XamlSourceFocusNavigationReason_Programmatic = 0,
    XamlSourceFocusNavigationReason_Restore = 1,
    XamlSourceFocusNavigationReason_First = 3,
    XamlSourceFocusNavigationReason_Last = 4,
    XamlSourceFocusNavigationReason_Left = 7,
    XamlSourceFocusNavigationReason_Up = 8,
    XamlSourceFocusNavigationReason_Right = 9,
    XamlSourceFocusNavigationReason_Down = 10,
};
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppExitedEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExitCode)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_get_ExitCode(This, value) \
    ((This)->lpVtbl->get_ExitCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppExitedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppManager[] = L"Windows.UI.Xaml.Hosting.IDesignerAppManager";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* add_DesignerAppExited)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesignerAppManager_Windows__CUI__CXaml__CHosting__CDesignerAppExitedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DesignerAppExited)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateNewViewAsync)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState initialViewState,
        struct __x_ABI_CWindows_CFoundation_CSize initialViewSize,
        __FIAsyncOperation_1_Windows__CUI__CXaml__CHosting__CDesignerAppView** operation);
    HRESULT (STDMETHODCALLTYPE* LoadObjectIntoAppAsync)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager* This,
        HSTRING dllName,
        GUID classId,
        HSTRING initializationData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_add_DesignerAppExited(This, handler, token) \
    ((This)->lpVtbl->add_DesignerAppExited(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_remove_DesignerAppExited(This, token) \
    ((This)->lpVtbl->remove_DesignerAppExited(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_CreateNewViewAsync(This, initialViewState, initialViewSize, operation) \
    ((This)->lpVtbl->CreateNewViewAsync(This, initialViewState, initialViewSize, operation))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_LoadObjectIntoAppAsync(This, dllName, classId, initializationData, operation) \
    ((This)->lpVtbl->LoadObjectIntoAppAsync(This, dllName, classId, initializationData, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppManagerFactory[] = L"Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory* This,
        HSTRING appUserModelId,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_Create(This, appUserModelId, value) \
    ((This)->lpVtbl->Create(This, appUserModelId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesignerAppView
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesignerAppView
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesignerAppView[] = L"Windows.UI.Xaml.Hosting.IDesignerAppView";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationViewId)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ViewState)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState* value);
    HRESULT (STDMETHODCALLTYPE* get_ViewSize)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* UpdateViewAsync)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CDesignerAppViewState viewState,
        struct __x_ABI_CWindows_CFoundation_CSize viewSize,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppViewVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_get_ApplicationViewId(This, value) \
    ((This)->lpVtbl->get_ApplicationViewId(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_get_ViewState(This, value) \
    ((This)->lpVtbl->get_ViewState(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_get_ViewSize(This, value) \
    ((This)->lpVtbl->get_ViewSize(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_UpdateViewAsync(This, viewState, viewSize, operation) \
    ((This)->lpVtbl->UpdateViewAsync(This, viewState, viewSize, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesignerAppView_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSource[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* get_HasFocus)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_TakeFocusRequested)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceTakeFocusRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TakeFocusRequested)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSource_Windows__CUI__CXaml__CHosting__CDesktopWindowXamlSourceGotFocusEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* NavigateFocus)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* request,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_get_HasFocus(This, value) \
    ((This)->lpVtbl->get_HasFocus(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_add_TakeFocusRequested(This, handler, token) \
    ((This)->lpVtbl->add_TakeFocusRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_remove_TakeFocusRequested(This, token) \
    ((This)->lpVtbl->remove_TakeFocusRequested(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_add_GotFocus(This, handler, token) \
    ((This)->lpVtbl->add_GotFocus(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_remove_GotFocus(This, token) \
    ((This)->lpVtbl->remove_GotFocus(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_NavigateFocus(This, request, result) \
    ((This)->lpVtbl->NavigateFocus(This, request, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceFactory[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceGotFocusEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceGotFocusEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IDesktopWindowXamlSourceTakeFocusRequestedEventArgs[] = L"Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIDesktopWindowXamlSourceTakeFocusRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreview[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreview";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetElementVisual)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CComposition_CIVisual** result);
    HRESULT (STDMETHODCALLTYPE* GetElementChildVisual)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CComposition_CIVisual** result);
    HRESULT (STDMETHODCALLTYPE* SetElementChildVisual)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* visual);
    HRESULT (STDMETHODCALLTYPE* GetScrollViewerManipulationPropertySet)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CControls_CIScrollViewer* scrollViewer,
        __x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetElementVisual(This, element, result) \
    ((This)->lpVtbl->GetElementVisual(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetElementChildVisual(This, element, result) \
    ((This)->lpVtbl->GetElementChildVisual(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_SetElementChildVisual(This, element, visual) \
    ((This)->lpVtbl->SetElementChildVisual(This, element, visual))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_GetScrollViewerManipulationPropertySet(This, scrollViewer, result) \
    ((This)->lpVtbl->GetScrollViewerManipulationPropertySet(This, scrollViewer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics2[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetImplicitShowAnimation)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase* animation);
    HRESULT (STDMETHODCALLTYPE* SetImplicitHideAnimation)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CComposition_CICompositionAnimationBase* animation);
    HRESULT (STDMETHODCALLTYPE* SetIsTranslationEnabled)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetPointerPositionPropertySet)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* targetElement,
        __x_ABI_CWindows_CUI_CComposition_CICompositionPropertySet** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_SetImplicitShowAnimation(This, element, animation) \
    ((This)->lpVtbl->SetImplicitShowAnimation(This, element, animation))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_SetImplicitHideAnimation(This, element, animation) \
    ((This)->lpVtbl->SetImplicitHideAnimation(This, element, animation))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_SetIsTranslationEnabled(This, element, value) \
    ((This)->lpVtbl->SetIsTranslationEnabled(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_GetPointerPositionPropertySet(This, targetElement, result) \
    ((This)->lpVtbl->GetPointerPositionPropertySet(This, targetElement, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IElementCompositionPreviewStatics3[] = L"Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetAppWindowContent)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* appWindow,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* xamlContent);
    HRESULT (STDMETHODCALLTYPE* GetAppWindowContent)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* appWindow,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_SetAppWindowContent(This, appWindow, xamlContent) \
    ((This)->lpVtbl->SetAppWindowContent(This, appWindow, xamlContent))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_GetAppWindowContent(This, appWindow, result) \
    ((This)->lpVtbl->GetAppWindowContent(This, appWindow, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIElementCompositionPreviewStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IWindowsXamlManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IWindowsXamlManager[] = L"Windows.UI.Xaml.Hosting.IWindowsXamlManager";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IWindowsXamlManagerStatics[] = L"Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InitializeForCurrentThread)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_InitializeForCurrentThread(This, result) \
    ((This)->lpVtbl->InitializeForCurrentThread(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIWindowsXamlManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationRequest[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason* value);
    HRESULT (STDMETHODCALLTYPE* get_HintRect)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_get_HintRect(This, value) \
    ((This)->lpVtbl->get_HintRect(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_get_CorrelationId(This, value) \
    ((This)->lpVtbl->get_CorrelationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationRequestFactory[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason reason,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithHintRect)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason reason,
        struct __x_ABI_CWindows_CFoundation_CRect hintRect,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithHintRectAndCorrelationId)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CHosting_CXamlSourceFocusNavigationReason reason,
        struct __x_ABI_CWindows_CFoundation_CRect hintRect,
        GUID correlationId,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_CreateInstance(This, reason, value) \
    ((This)->lpVtbl->CreateInstance(This, reason, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_CreateInstanceWithHintRect(This, reason, hintRect, value) \
    ((This)->lpVtbl->CreateInstanceWithHintRect(This, reason, hintRect, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_CreateInstanceWithHintRectAndCorrelationId(This, reason, hintRect, correlationId, value) \
    ((This)->lpVtbl->CreateInstanceWithHintRectAndCorrelationId(This, reason, hintRect, correlationId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationResult[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WasFocusMoved)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_get_WasFocusMoved(This, value) \
    ((This)->lpVtbl->get_WasFocusMoved(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlSourceFocusNavigationResultFactory[] = L"Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory* This,
        boolean focusMoved,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_CreateInstance(This, focusMoved, value) \
    ((This)->lpVtbl->CreateInstance(This, focusMoved, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlSourceFocusNavigationResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenter
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenter[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenter";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RootElement)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** value);
    HRESULT (STDMETHODCALLTYPE* put_RootElement)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* get_ThemeKey)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ThemeKey)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ThemeResourcesXaml)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ThemeResourcesXaml)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetSize)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This,
        INT32 width,
        INT32 height);
    HRESULT (STDMETHODCALLTYPE* Render)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This);
    HRESULT (STDMETHODCALLTYPE* Present)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_get_RootElement(This, value) \
    ((This)->lpVtbl->get_RootElement(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_put_RootElement(This, value) \
    ((This)->lpVtbl->put_RootElement(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_get_ThemeKey(This, value) \
    ((This)->lpVtbl->get_ThemeKey(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_put_ThemeKey(This, value) \
    ((This)->lpVtbl->put_ThemeKey(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_get_ThemeResourcesXaml(This, value) \
    ((This)->lpVtbl->get_ThemeResourcesXaml(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_put_ThemeResourcesXaml(This, value) \
    ((This)->lpVtbl->put_ThemeResourcesXaml(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_SetSize(This, width, height) \
    ((This)->lpVtbl->SetSize(This, width, height))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_Render(This) \
    ((This)->lpVtbl->Render(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_Present(This) \
    ((This)->lpVtbl->Present(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHostVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ResolveFileResource)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* This,
        HSTRING path,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHostVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHostVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_ResolveFileResource(This, path, result) \
    ((This)->lpVtbl->ResolveFileResource(This, path, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost2[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetGenericXamlFilePath)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_GetGenericXamlFilePath(This, result) \
    ((This)->lpVtbl->GetGenericXamlFilePath(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterHost3[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ResolveDictionaryResource)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3* This,
        __x_ABI_CWindows_CUI_CXaml_CIResourceDictionary* dictionary,
        IInspectable* dictionaryKey,
        IInspectable* suggestedValue,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_ResolveDictionaryResource(This, dictionary, dictionaryKey, suggestedValue, result) \
    ((This)->lpVtbl->ResolveDictionaryResource(This, dictionary, dictionaryKey, suggestedValue, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost3_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterStatics[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CompleteTimelinesAutomatically)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CompleteTimelinesAutomatically)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetHost)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterHost* host);
    HRESULT (STDMETHODCALLTYPE* NotifyWindowSizeChanged)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_get_CompleteTimelinesAutomatically(This, value) \
    ((This)->lpVtbl->get_CompleteTimelinesAutomatically(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_put_CompleteTimelinesAutomatically(This, value) \
    ((This)->lpVtbl->put_CompleteTimelinesAutomatically(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_SetHost(This, host) \
    ((This)->lpVtbl->SetHost(This, host))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_NotifyWindowSizeChanged(This) \
    ((This)->lpVtbl->NotifyWindowSizeChanged(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Hosting_IXamlUIPresenterStatics2[] = L"Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFlyoutPlacementTargetInfo)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement* placementTarget,
        enum __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode preferredPlacement,
        enum __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode* targetPreferredPlacement,
        boolean* allowFallbacks,
        struct __x_ABI_CWindows_CFoundation_CRect* returnValue);
    HRESULT (STDMETHODCALLTYPE* GetFlyoutPlacement)(__x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2* This,
        struct __x_ABI_CWindows_CFoundation_CRect placementTargetBounds,
        struct __x_ABI_CWindows_CFoundation_CSize controlSize,
        struct __x_ABI_CWindows_CFoundation_CSize minControlSize,
        struct __x_ABI_CWindows_CFoundation_CRect containerRect,
        enum __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode targetPreferredPlacement,
        boolean allowFallbacks,
        enum __x_ABI_CWindows_CUI_CXaml_CControls_CPrimitives_CFlyoutPlacementMode* chosenPlacement,
        struct __x_ABI_CWindows_CFoundation_CRect* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_GetFlyoutPlacementTargetInfo(This, placementTarget, preferredPlacement, targetPreferredPlacement, allowFallbacks, returnValue) \
    ((This)->lpVtbl->GetFlyoutPlacementTargetInfo(This, placementTarget, preferredPlacement, targetPreferredPlacement, allowFallbacks, returnValue))

#define __x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_GetFlyoutPlacement(This, placementTargetBounds, controlSize, minControlSize, containerRect, targetPreferredPlacement, allowFallbacks, chosenPlacement, returnValue) \
    ((This)->lpVtbl->GetFlyoutPlacement(This, placementTargetBounds, controlSize, minControlSize, containerRect, targetPreferredPlacement, allowFallbacks, chosenPlacement, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CHosting_CIXamlUIPresenterStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppExitedEventArgs[] = L"Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory interface starting with version 2.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppManager ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppManager[] = L"Windows.UI.Xaml.Hosting.DesignerAppManager";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesignerAppView
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesignerAppView ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppView_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesignerAppView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesignerAppView[] = L"Windows.UI.Xaml.Hosting.DesignerAppView";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSource
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSource[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceGotFocusEventArgs[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_DesktopWindowXamlSourceTakeFocusRequestedEventArgs[] = L"Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.ElementCompositionPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IElementCompositionPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_ElementCompositionPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_ElementCompositionPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_ElementCompositionPreview[] = L"Windows.UI.Xaml.Hosting.ElementCompositionPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Xaml.Hosting.WindowsXamlManager
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IWindowsXamlManager ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_WindowsXamlManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_WindowsXamlManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_WindowsXamlManager[] = L"Windows.UI.Xaml.Hosting.WindowsXamlManager";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationRequest[] = L"Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory interface starting with version 3.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlSourceFocusNavigationResult[] = L"Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Xaml.Hosting.XamlUIPresenter
 *
 * Introduced to Windows.UI.Xaml.Hosting.HostingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics interface starting with version 1.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2 interface starting with version 1.0 of the Windows.UI.Xaml.Hosting.HostingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Hosting.IXamlUIPresenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlUIPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Hosting_XamlUIPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Hosting_XamlUIPresenter[] = L"Windows.UI.Xaml.Hosting.XamlUIPresenter";
#endif
#endif // WINDOWS_UI_XAML_HOSTING_HOSTINGCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Examl2Ehosting_p_h__

#endif // __windows2Eui2Examl2Ehosting_h__
