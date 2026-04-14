
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
#ifndef __windows2Eui2Examl2Enavigation_h__
#define __windows2Eui2Examl2Enavigation_h__
#ifndef __windows2Eui2Examl2Enavigation_p_h__
#define __windows2Eui2Examl2Enavigation_p_h__


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
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Interop.h"
#include "Windows.UI.Xaml.Media.Animation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface ILoadCompletedEventHandler;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler ABI::Windows::UI::Xaml::Navigation::ILoadCompletedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigatedEventHandler;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler ABI::Windows::UI::Xaml::Navigation::INavigatedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigatingCancelEventHandler;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler ABI::Windows::UI::Xaml::Navigation::INavigatingCancelEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigationFailedEventHandler;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler ABI::Windows::UI::Xaml::Navigation::INavigationFailedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigationStoppedEventHandler;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler ABI::Windows::UI::Xaml::Navigation::INavigationStoppedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface IFrameNavigationOptions;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions ABI::Windows::UI::Xaml::Navigation::IFrameNavigationOptions

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface IFrameNavigationOptionsFactory;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory ABI::Windows::UI::Xaml::Navigation::IFrameNavigationOptionsFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigatingCancelEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs ABI::Windows::UI::Xaml::Navigation::INavigatingCancelEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigatingCancelEventArgs2;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2 ABI::Windows::UI::Xaml::Navigation::INavigatingCancelEventArgs2

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigationEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs ABI::Windows::UI::Xaml::Navigation::INavigationEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigationEventArgs2;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2 ABI::Windows::UI::Xaml::Navigation::INavigationEventArgs2

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface INavigationFailedEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs ABI::Windows::UI::Xaml::Navigation::INavigationFailedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface IPageStackEntry;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry ABI::Windows::UI::Xaml::Navigation::IPageStackEntry

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface IPageStackEntryFactory;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory ABI::Windows::UI::Xaml::Navigation::IPageStackEntryFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    interface IPageStackEntryStatics;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics ABI::Windows::UI::Xaml::Navigation::IPageStackEntryStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
        namespace UI {
            namespace Xaml {
                class DependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty ABI::Windows::UI::Xaml::IDependencyProperty

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    typedef struct TypeName TypeName;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Animation {
                        class NavigationTransitionInfo;
                    } /* Animation */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Animation {
                        interface INavigationTransitionInfo;
                    } /* Animation */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    typedef enum NavigationMode : int NavigationMode;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    class FrameNavigationOptions;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    class NavigatingCancelEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    class NavigationEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    class NavigationFailedEventArgs;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    class PageStackEntry;
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Navigation.NavigationCacheMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    enum NavigationCacheMode : int
                    {
                        NavigationCacheMode_Disabled = 0,
                        NavigationCacheMode_Required = 1,
                        NavigationCacheMode_Enabled = 2,
                    };
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Navigation.NavigationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    enum NavigationMode : int
                    {
                        NavigationMode_New = 0,
                        NavigationMode_Back = 1,
                        NavigationMode_Forward = 2,
                        NavigationMode_Refresh = 3,
                    };
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.LoadCompletedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("aebaf785-43fc-4e2c-95c3-97ae84eabc8e")
                    ILoadCompletedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Navigation::INavigationEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILoadCompletedEventHandler = __uuidof(ILoadCompletedEventHandler);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("7bd1cf54-23cf-4cce-b2f5-4ce78d96896e")
                    INavigatedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Navigation::INavigationEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigatedEventHandler = __uuidof(INavigatedEventHandler);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigatingCancelEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("75d6a78f-a302-4489-9898-24ea49182910")
                    INavigatingCancelEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Navigation::INavigatingCancelEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigatingCancelEventHandler = __uuidof(INavigatingCancelEventHandler);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigationFailedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("4dab4671-12b2-43c7-b892-9be2dcd3e88d")
                    INavigationFailedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Navigation::INavigationFailedEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigationFailedEventHandler = __uuidof(INavigationFailedEventHandler);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigationStoppedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("f0117ddb-12fa-4d8d-8b26-b383d09c2b3c")
                    INavigationStoppedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Navigation::INavigationEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigationStoppedEventHandler = __uuidof(INavigationStoppedEventHandler);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IFrameNavigationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IFrameNavigationOptions[] = L"Windows.UI.Xaml.Navigation.IFrameNavigationOptions";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("b539ad2a-9fb7-520a-8f41-57a50c59cf92")
                    IFrameNavigationOptions : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsNavigationStackEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsNavigationStackEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TransitionInfoOverride(
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TransitionInfoOverride(
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameNavigationOptions = __uuidof(IFrameNavigationOptions);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IFrameNavigationOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IFrameNavigationOptionsFactory[] = L"Windows.UI.Xaml.Navigation.IFrameNavigationOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("d4681e41-7e6d-5c7c-aca0-478681cc6fce")
                    IFrameNavigationOptionsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Navigation::IFrameNavigationOptions** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameNavigationOptionsFactory = __uuidof(IFrameNavigationOptionsFactory);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigatingCancelEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("fd1d67ae-eafb-4079-be80-6dc92a03aedf")
                    INavigatingCancelEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Cancel(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Cancel(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NavigationMode(
                            ABI::Windows::UI::Xaml::Navigation::NavigationMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePageType(
                            ABI::Windows::UI::Xaml::Interop::TypeName* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigatingCancelEventArgs = __uuidof(INavigatingCancelEventArgs);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigatingCancelEventArgs2[] = L"Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("5407b704-8147-4343-838f-dd1ee908c137")
                    INavigatingCancelEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Parameter(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NavigationTransitionInfo(
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigatingCancelEventArgs2 = __uuidof(INavigatingCancelEventArgs2);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigationEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("b6aa9834-6691-44d1-bdf7-58820c27b0d0")
                    INavigationEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Content(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Parameter(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePageType(
                            ABI::Windows::UI::Xaml::Interop::TypeName* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NavigationMode(
                            ABI::Windows::UI::Xaml::Navigation::NavigationMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigationEventArgs = __uuidof(INavigationEventArgs);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationEventArgs2[] = L"Windows.UI.Xaml.Navigation.INavigationEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("dbff71d9-979a-4b2e-a49b-3bb17fdef574")
                    INavigationEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NavigationTransitionInfo(
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigationEventArgs2 = __uuidof(INavigationEventArgs2);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationFailedEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigationFailedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("11c1dff7-36c2-4102-b2ef-0217a97289b3")
                    INavigationFailedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Exception(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePageType(
                            ABI::Windows::UI::Xaml::Interop::TypeName* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INavigationFailedEventArgs = __uuidof(INavigationFailedEventArgs);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntry[] = L"Windows.UI.Xaml.Navigation.IPageStackEntry";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("ef8814a6-9388-4aca-8572-405194069080")
                    IPageStackEntry : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePageType(
                            ABI::Windows::UI::Xaml::Interop::TypeName* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Parameter(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NavigationTransitionInfo(
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPageStackEntry = __uuidof(IPageStackEntry);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntryFactory[] = L"Windows.UI.Xaml.Navigation.IPageStackEntryFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("4454048a-a8b9-4f78-9b84-1f51f58851ff")
                    IPageStackEntryFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::UI::Xaml::Interop::TypeName sourcePageType,
                            IInspectable* parameter,
                            ABI::Windows::UI::Xaml::Media::Animation::INavigationTransitionInfo* navigationTransitionInfo,
                            ABI::Windows::UI::Xaml::Navigation::IPageStackEntry** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPageStackEntryFactory = __uuidof(IPageStackEntryFactory);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntryStatics[] = L"Windows.UI.Xaml.Navigation.IPageStackEntryStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Navigation {
                    MIDL_INTERFACE("aceff8e3-246c-4033-9f01-01cb0da5254e")
                    IPageStackEntryStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePageTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPageStackEntryStatics = __uuidof(IPageStackEntryStatics);
                } /* Navigation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.IFrameNavigationOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_FrameNavigationOptions_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_FrameNavigationOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_FrameNavigationOptions[] = L"Windows.UI.Xaml.Navigation.FrameNavigationOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs ** Default Interface **
 *    Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigationEventArgs ** Default Interface **
 *    Windows.UI.Xaml.Navigation.INavigationEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigationEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigationFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigationFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigationFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.PageStackEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Navigation.IPageStackEntryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Navigation.IPageStackEntryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.IPageStackEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_PageStackEntry_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_PageStackEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_PageStackEntry[] = L"Windows.UI.Xaml.Navigation.PageStackEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2 __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2 __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationMode __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationMode;

/*
 *
 * Struct Windows.UI.Xaml.Navigation.NavigationCacheMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationCacheMode
{
    NavigationCacheMode_Disabled = 0,
    NavigationCacheMode_Required = 1,
    NavigationCacheMode_Enabled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Navigation.NavigationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationMode
{
    NavigationMode_New = 0,
    NavigationMode_Back = 1,
    NavigationMode_Forward = 2,
    NavigationMode_Refresh = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.LoadCompletedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CILoadCompletedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigatingCancelEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigationFailedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Navigation.NavigationStoppedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationStoppedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IFrameNavigationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IFrameNavigationOptions[] = L"Windows.UI.Xaml.Navigation.IFrameNavigationOptions";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsNavigationStackEnabled)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsNavigationStackEnabled)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TransitionInfoOverride)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo** value);
    HRESULT (STDMETHODCALLTYPE* put_TransitionInfoOverride)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_get_IsNavigationStackEnabled(This, value) \
    ((This)->lpVtbl->get_IsNavigationStackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_put_IsNavigationStackEnabled(This, value) \
    ((This)->lpVtbl->put_IsNavigationStackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_get_TransitionInfoOverride(This, value) \
    ((This)->lpVtbl->get_TransitionInfoOverride(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_put_TransitionInfoOverride(This, value) \
    ((This)->lpVtbl->put_TransitionInfoOverride(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IFrameNavigationOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IFrameNavigationOptionsFactory[] = L"Windows.UI.Xaml.Navigation.IFrameNavigationOptionsFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIFrameNavigationOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigatingCancelEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Cancel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Cancel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NavigationMode)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationMode* value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePageType)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_get_Cancel(This, value) \
    ((This)->lpVtbl->get_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_put_Cancel(This, value) \
    ((This)->lpVtbl->put_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_get_NavigationMode(This, value) \
    ((This)->lpVtbl->get_NavigationMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_get_SourcePageType(This, value) \
    ((This)->lpVtbl->get_SourcePageType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigatingCancelEventArgs2[] = L"Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Parameter)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_NavigationTransitionInfo)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_get_Parameter(This, value) \
    ((This)->lpVtbl->get_Parameter(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_get_NavigationTransitionInfo(This, value) \
    ((This)->lpVtbl->get_NavigationTransitionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigatingCancelEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigationEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_Parameter)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePageType)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName* value);
    HRESULT (STDMETHODCALLTYPE* get_NavigationMode)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CNavigation_CNavigationMode* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_get_Parameter(This, value) \
    ((This)->lpVtbl->get_Parameter(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_get_SourcePageType(This, value) \
    ((This)->lpVtbl->get_SourcePageType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_get_NavigationMode(This, value) \
    ((This)->lpVtbl->get_NavigationMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationEventArgs2[] = L"Windows.UI.Xaml.Navigation.INavigationEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NavigationTransitionInfo)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_get_NavigationTransitionInfo(This, value) \
    ((This)->lpVtbl->get_NavigationTransitionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.INavigationFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.NavigationFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_INavigationFailedEventArgs[] = L"Windows.UI.Xaml.Navigation.INavigationFailedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Exception)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePageType)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_get_Exception(This, value) \
    ((This)->lpVtbl->get_Exception(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_get_SourcePageType(This, value) \
    ((This)->lpVtbl->get_SourcePageType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CINavigationFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntry[] = L"Windows.UI.Xaml.Navigation.IPageStackEntry";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourcePageType)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName* value);
    HRESULT (STDMETHODCALLTYPE* get_Parameter)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_NavigationTransitionInfo)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_get_SourcePageType(This, value) \
    ((This)->lpVtbl->get_SourcePageType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_get_Parameter(This, value) \
    ((This)->lpVtbl->get_Parameter(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_get_NavigationTransitionInfo(This, value) \
    ((This)->lpVtbl->get_NavigationTransitionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntryFactory[] = L"Windows.UI.Xaml.Navigation.IPageStackEntryFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName sourcePageType,
        IInspectable* parameter,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CAnimation_CINavigationTransitionInfo* navigationTransitionInfo,
        __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntry** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_CreateInstance(This, sourcePageType, parameter, navigationTransitionInfo, value) \
    ((This)->lpVtbl->CreateInstance(This, sourcePageType, parameter, navigationTransitionInfo, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Navigation.IPageStackEntryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Navigation.PageStackEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Navigation_IPageStackEntryStatics[] = L"Windows.UI.Xaml.Navigation.IPageStackEntryStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourcePageTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_get_SourcePageTypeProperty(This, value) \
    ((This)->lpVtbl->get_SourcePageTypeProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CNavigation_CIPageStackEntryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.FrameNavigationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.IFrameNavigationOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_FrameNavigationOptions_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_FrameNavigationOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_FrameNavigationOptions[] = L"Windows.UI.Xaml.Navigation.FrameNavigationOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs ** Default Interface **
 *    Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigatingCancelEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigationEventArgs ** Default Interface **
 *    Windows.UI.Xaml.Navigation.INavigationEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigationEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.NavigationFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.INavigationFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_NavigationFailedEventArgs[] = L"Windows.UI.Xaml.Navigation.NavigationFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Navigation.PageStackEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Navigation.IPageStackEntryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Navigation.IPageStackEntryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Navigation.IPageStackEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Navigation_PageStackEntry_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Navigation_PageStackEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Navigation_PageStackEntry[] = L"Windows.UI.Xaml.Navigation.PageStackEntry";
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
#endif // __windows2Eui2Examl2Enavigation_p_h__

#endif // __windows2Eui2Examl2Enavigation_h__
