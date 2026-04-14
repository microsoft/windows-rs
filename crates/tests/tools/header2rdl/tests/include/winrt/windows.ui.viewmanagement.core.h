
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
#ifndef __windows2Eui2Eviewmanagement2Ecore_h__
#define __windows2Eui2Eviewmanagement2Ecore_h__
#ifndef __windows2Eui2Eviewmanagement2Ecore_p_h__
#define __windows2Eui2Eviewmanagement2Ecore_p_h__


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
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreFrameworkInputView;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputView

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreFrameworkInputViewAnimationStartingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputViewAnimationStartingEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreFrameworkInputViewOcclusionsChangedEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputViewOcclusionsChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreFrameworkInputViewStatics;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputViewStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputView;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView ABI::Windows::UI::ViewManagement::Core::ICoreInputView

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputView2;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2 ABI::Windows::UI::ViewManagement::Core::ICoreInputView2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputView3;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3 ABI::Windows::UI::ViewManagement::Core::ICoreInputView3

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputView4;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4 ABI::Windows::UI::ViewManagement::Core::ICoreInputView4

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputView5;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5 ABI::Windows::UI::ViewManagement::Core::ICoreInputView5

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewAnimationStartingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreInputViewAnimationStartingEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewHidingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreInputViewHidingEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewOcclusion;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusion

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewOcclusionsChangedEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusionsChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewShowingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreInputViewShowingEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewStatics;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics ABI::Windows::UI::ViewManagement::Core::ICoreInputViewStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewStatics2;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2 ABI::Windows::UI::ViewManagement::Core::ICoreInputViewStatics2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface ICoreInputViewTransferringXYFocusEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs ABI::Windows::UI::ViewManagement::Core::ICoreInputViewTransferringXYFocusEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface IUISettingsController;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController ABI::Windows::UI::ViewManagement::Core::IUISettingsController

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    interface IUISettingsControllerStatics;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics ABI::Windows::UI::ViewManagement::Core::IUISettingsControllerStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class UISettingsController;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bc3153c5-58d0-5b0b-a9a5-1b7cc2faef5f"))
IAsyncOperation<ABI::Windows::UI::ViewManagement::Core::UISettingsController*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::UISettingsController*, ABI::Windows::UI::ViewManagement::Core::IUISettingsController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.ViewManagement.Core.UISettingsController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::ViewManagement::Core::UISettingsController*> __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_t;
#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88dda56b-dfc1-5165-ac13-fc9868bb4ab7"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::ViewManagement::Core::UISettingsController*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::UISettingsController*, ABI::Windows::UI::ViewManagement::Core::IUISettingsController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.ViewManagement.Core.UISettingsController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::ViewManagement::Core::UISettingsController*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewOcclusion;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#define DEF___FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5bb57354-4f40-5ef3-a5d1-6a6049f905a1"))
IIterator<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.ViewManagement.Core.CoreInputViewOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t;
#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#define DEF___FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0a11958b-63da-5566-913a-180550dad26a"))
IIterable<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.ViewManagement.Core.CoreInputViewOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t;
#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#define DEF___FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e0963578-a246-5680-86d1-27519423e212"))
IVectorView<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.ViewManagement.Core.CoreInputViewOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusion*> __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t;
#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreFrameworkInputView;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreFrameworkInputViewAnimationStartingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cfd46669-0e5a-5c60-a8a2-dc2d899789da"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewAnimationStartingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewAnimationStartingEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputViewAnimationStartingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreFrameworkInputView, Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewAnimationStartingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreFrameworkInputViewOcclusionsChangedEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d081e63b-b7c7-5859-8a08-bb19bf0c90b2"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewOcclusionsChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewOcclusionsChangedEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputViewOcclusionsChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreFrameworkInputView, Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputView*, ABI::Windows::UI::ViewManagement::Core::CoreFrameworkInputViewOcclusionsChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputView;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2bc0e7f6-c772-56e0-9439-650666c78d0c"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewAnimationStartingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("070cc1cf-2c3a-5e23-a0aa-0dabe6d4d27e"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewAnimationStartingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewAnimationStartingEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewAnimationStartingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewAnimationStartingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewHidingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3188afc6-4b93-58a0-b5b0-cbed65be0c7e"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewHidingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewHidingEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewHidingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewHidingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewOcclusionsChangedEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5adecf04-edd1-5133-abc7-582a027f09bb"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusionsChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusionsChangedEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewOcclusionsChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusionsChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewShowingEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a300e2f9-737b-5330-8ea2-68e7a3aaedb2"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewShowingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewShowingEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewShowingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewShowingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    class CoreInputViewTransferringXYFocusEventArgs;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("881e8198-7ff6-5cd9-8a64-6dd4292267ad"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewTransferringXYFocusEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::ICoreInputView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::Core::CoreInputViewTransferringXYFocusEventArgs*, ABI::Windows::UI::ViewManagement::Core::ICoreInputViewTransferringXYFocusEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.Core.CoreInputView, Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::Core::CoreInputView*, ABI::Windows::UI::ViewManagement::Core::CoreInputViewTransferringXYFocusEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
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
            namespace ViewManagement {
                namespace Core {
                    typedef enum CoreInputViewKind : int CoreInputViewKind;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    typedef enum CoreInputViewOcclusionKind : int CoreInputViewOcclusionKind;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    typedef enum CoreInputViewXYFocusTransferDirection : int CoreInputViewXYFocusTransferDirection;
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    enum CoreInputViewKind : int
                    {
                        CoreInputViewKind_Default = 0,
                        CoreInputViewKind_Keyboard = 1,
                        CoreInputViewKind_Handwriting = 2,
                        CoreInputViewKind_Emoji = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        CoreInputViewKind_Symbols = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        CoreInputViewKind_Clipboard = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        CoreInputViewKind_Dictation = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
                        CoreInputViewKind_Gamepad = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
                    };
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    enum CoreInputViewOcclusionKind : int
                    {
                        CoreInputViewOcclusionKind_Docked = 0,
                        CoreInputViewOcclusionKind_Floating = 1,
                        CoreInputViewOcclusionKind_Overlay = 2,
                    };
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    enum CoreInputViewXYFocusTransferDirection : int
                    {
                        CoreInputViewXYFocusTransferDirection_Up = 0,
                        CoreInputViewXYFocusTransferDirection_Right = 1,
                        CoreInputViewXYFocusTransferDirection_Down = 2,
                        CoreInputViewXYFocusTransferDirection_Left = 3,
                    };
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputView[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("d77c94ae-46b8-5d4a-9489-8ddec3d639a6")
                    ICoreFrameworkInputView : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_PrimaryViewAnimationStarting(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrimaryViewAnimationStarting(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_OcclusionsChanged(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_OcclusionsChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreFrameworkInputView = __uuidof(ICoreFrameworkInputView);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("c0ec901c-bba4-501b-ae8b-65c9e756a719")
                    ICoreFrameworkInputViewAnimationStartingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Occlusions(
                            __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FrameworkAnimationRecommended(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AnimationDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreFrameworkInputViewAnimationStartingEventArgs = __uuidof(ICoreFrameworkInputViewAnimationStartingEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("f36f4949-c82c-53d1-a75d-2b2baf0d9b0d")
                    ICoreFrameworkInputViewOcclusionsChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Occlusions(
                            __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreFrameworkInputViewOcclusionsChangedEventArgs = __uuidof(ICoreFrameworkInputViewOcclusionsChangedEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewStatics[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("6eebd9b6-eac2-5f8b-975f-772ee3e42eeb")
                    ICoreFrameworkInputViewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForUIContext(
                            ABI::Windows::UI::IUIContext* context,
                            ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputView** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::UI::ViewManagement::Core::ICoreFrameworkInputView** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreFrameworkInputViewStatics = __uuidof(ICoreFrameworkInputViewStatics);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView[] = L"Windows.UI.ViewManagement.Core.ICoreInputView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("c770cd7a-7001-4c32-bf94-25c1f554cbf1")
                    ICoreInputView : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_OcclusionsChanged(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_OcclusionsChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCoreInputViewOcclusions(
                            __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryShowPrimaryView(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryHidePrimaryView(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputView = __uuidof(ICoreInputView);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView2[] = L"Windows.UI.ViewManagement.Core.ICoreInputView2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("0ed726c1-e09a-4ae8-aedf-dfa4857d1a01")
                    ICoreInputView2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_XYFocusTransferringFromPrimaryView(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_XYFocusTransferringFromPrimaryView(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_XYFocusTransferredToPrimaryView(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_XYFocusTransferredToPrimaryView(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryTransferXYFocusToPrimaryView(
                            ABI::Windows::Foundation::Rect origin,
                            ABI::Windows::UI::ViewManagement::Core::CoreInputViewXYFocusTransferDirection direction,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputView2 = __uuidof(ICoreInputView2);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView3[] = L"Windows.UI.ViewManagement.Core.ICoreInputView3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("bc941653-3ab9-4849-8f58-46e7f0353cfc")
                    ICoreInputView3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryShow(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryShowWithKind(
                            ABI::Windows::UI::ViewManagement::Core::CoreInputViewKind type,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryHide(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputView3 = __uuidof(ICoreInputView3);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView4[] = L"Windows.UI.ViewManagement.Core.ICoreInputView4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("002863d6-d9ef-57eb-8cef-77f6ce1b7ee7")
                    ICoreInputView4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_PrimaryViewShowing(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrimaryViewShowing(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PrimaryViewHiding(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrimaryViewHiding(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputView4 = __uuidof(ICoreInputView4);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView5[] = L"Windows.UI.ViewManagement.Core.ICoreInputView5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("136316e0-c6d5-5c57-811e-1ad8a99ba6ab")
                    ICoreInputView5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsKindSupported(
                            ABI::Windows::UI::ViewManagement::Core::CoreInputViewKind type,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SupportedKindsChanged(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SupportedKindsChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PrimaryViewAnimationStarting(
                            __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrimaryViewAnimationStarting(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputView5 = __uuidof(ICoreInputView5);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("a9144af2-b55c-5ea1-b8ab-5340f3e94897")
                    ICoreInputViewAnimationStartingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Occlusions(
                            __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AnimationDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewAnimationStartingEventArgs = __uuidof(ICoreInputViewAnimationStartingEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewHidingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("eada47bd-bac5-5336-848d-41083584daad")
                    ICoreInputViewHidingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryCancel(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewHidingEventArgs = __uuidof(ICoreInputViewHidingEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewOcclusion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewOcclusion[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("cc36ce06-3865-4177-b5f5-8b65e0b9ce84")
                    ICoreInputViewOcclusion : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OccludingRect(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OcclusionKind(
                            ABI::Windows::UI::ViewManagement::Core::CoreInputViewOcclusionKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewOcclusion = __uuidof(ICoreInputViewOcclusion);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("be1027e8-b3ee-4df7-9554-89cdc66082c2")
                    ICoreInputViewOcclusionsChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Occlusions(
                            __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewOcclusionsChangedEventArgs = __uuidof(ICoreInputViewOcclusionsChangedEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewShowingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("ca52261b-fb9e-5daf-a98c-262b8b76af50")
                    ICoreInputViewShowingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryCancel(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewShowingEventArgs = __uuidof(ICoreInputViewShowingEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewStatics[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("7d9b97cd-edbe-49cf-a54f-337de052907f")
                    ICoreInputViewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::UI::ViewManagement::Core::ICoreInputView** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewStatics = __uuidof(ICoreInputViewStatics);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewStatics2[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("7ebc0862-d049-4e52-87b0-1e90e98c49ed")
                    ICoreInputViewStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForUIContext(
                            ABI::Windows::UI::IUIContext* context,
                            ABI::Windows::UI::ViewManagement::Core::ICoreInputView** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewStatics2 = __uuidof(ICoreInputViewStatics2);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewTransferringXYFocusEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("04de169f-ba02-4850-8b55-d82d03ba6d7f")
                    ICoreInputViewTransferringXYFocusEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Origin(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Direction(
                            ABI::Windows::UI::ViewManagement::Core::CoreInputViewXYFocusTransferDirection* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TransferHandled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TransferHandled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeepPrimaryViewVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeepPrimaryViewVisible(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreInputViewTransferringXYFocusEventArgs = __uuidof(ICoreInputViewTransferringXYFocusEventArgs);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.IUISettingsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.UISettingsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_IUISettingsController[] = L"Windows.UI.ViewManagement.Core.IUISettingsController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("78a51ac4-15c0-5a1b-a75b-acbf9cb8bb9e")
                    IUISettingsController : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetAdvancedEffectsEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAnimationsEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAutoHideScrollBars(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetMessageDuration(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetTextScaleFactor(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUISettingsController = __uuidof(IUISettingsController);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.IUISettingsControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.UISettingsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_IUISettingsControllerStatics[] = L"Windows.UI.ViewManagement.Core.IUISettingsControllerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                namespace Core {
                    MIDL_INTERFACE("eb3c68cc-c220-578c-8119-7db324ed26a6")
                    IUISettingsControllerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestDefaultAsync(
                            __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUISettingsControllerStatics = __uuidof(IUISettingsControllerStatics);
                } /* Core */
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputView ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputView[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreInputViewStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreInputViewStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputView ** Default Interface **
 *    Windows.UI.ViewManagement.Core.ICoreInputView2
 *    Windows.UI.ViewManagement.Core.ICoreInputView3
 *    Windows.UI.ViewManagement.Core.ICoreInputView4
 *    Windows.UI.ViewManagement.Core.ICoreInputView5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputView[] = L"Windows.UI.ViewManagement.Core.CoreInputView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion[] = L"Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.ViewManagement.Core.UISettingsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.IUISettingsControllerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.IUISettingsController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_UISettingsController_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_UISettingsController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_UISettingsController[] = L"Windows.UI.ViewManagement.Core.UISettingsController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2 __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3 __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4 __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5 __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2 __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController;

typedef struct __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* This,
        __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CViewManagement__CCore__CUISettingsController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

typedef struct __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl;

interface __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

typedef struct __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        __FIIterator_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl;

interface __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion;

typedef struct __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl;

interface __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewKind __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewKind;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewOcclusionKind __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewOcclusionKind;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewXYFocusTransferDirection __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewXYFocusTransferDirection;

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewKind
{
    CoreInputViewKind_Default = 0,
    CoreInputViewKind_Keyboard = 1,
    CoreInputViewKind_Handwriting = 2,
    CoreInputViewKind_Emoji = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    CoreInputViewKind_Symbols = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    CoreInputViewKind_Clipboard = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    CoreInputViewKind_Dictation = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
    CoreInputViewKind_Gamepad = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewOcclusionKind
{
    CoreInputViewOcclusionKind_Docked = 0,
    CoreInputViewOcclusionKind_Floating = 1,
    CoreInputViewOcclusionKind_Overlay = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewXYFocusTransferDirection
{
    CoreInputViewXYFocusTransferDirection_Up = 0,
    CoreInputViewXYFocusTransferDirection_Right = 1,
    CoreInputViewXYFocusTransferDirection_Down = 2,
    CoreInputViewXYFocusTransferDirection_Left = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputView[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputView";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PrimaryViewAnimationStarting)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewAnimationStartingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrimaryViewAnimationStarting)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_OcclusionsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputView_Windows__CUI__CViewManagement__CCore__CCoreFrameworkInputViewOcclusionsChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OcclusionsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_add_PrimaryViewAnimationStarting(This, handler, token) \
    ((This)->lpVtbl->add_PrimaryViewAnimationStarting(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_remove_PrimaryViewAnimationStarting(This, token) \
    ((This)->lpVtbl->remove_PrimaryViewAnimationStarting(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_add_OcclusionsChanged(This, handler, token) \
    ((This)->lpVtbl->add_OcclusionsChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_remove_OcclusionsChanged(This, token) \
    ((This)->lpVtbl->remove_OcclusionsChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Occlusions)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value);
    HRESULT (STDMETHODCALLTYPE* get_FrameworkAnimationRecommended)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AnimationDuration)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_get_Occlusions(This, value) \
    ((This)->lpVtbl->get_Occlusions(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_get_FrameworkAnimationRecommended(This, value) \
    ((This)->lpVtbl->get_FrameworkAnimationRecommended(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_get_AnimationDuration(This, value) \
    ((This)->lpVtbl->get_AnimationDuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Occlusions)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_get_Occlusions(This, value) \
    ((This)->lpVtbl->get_Occlusions(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreFrameworkInputViewStatics[] = L"Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUIContext)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        __x_ABI_CWindows_CUI_CIUIContext* context,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView** result);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputView** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_GetForUIContext(This, context, result) \
    ((This)->lpVtbl->GetForUIContext(This, context, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreFrameworkInputViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView[] = L"Windows.UI.ViewManagement.Core.ICoreInputView";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_OcclusionsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusionsChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OcclusionsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetCoreInputViewOcclusions)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** result);
    HRESULT (STDMETHODCALLTYPE* TryShowPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryHidePrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_add_OcclusionsChanged(This, handler, token) \
    ((This)->lpVtbl->add_OcclusionsChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_remove_OcclusionsChanged(This, token) \
    ((This)->lpVtbl->remove_OcclusionsChanged(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_GetCoreInputViewOcclusions(This, result) \
    ((This)->lpVtbl->GetCoreInputViewOcclusions(This, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_TryShowPrimaryView(This, result) \
    ((This)->lpVtbl->TryShowPrimaryView(This, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_TryHidePrimaryView(This, result) \
    ((This)->lpVtbl->TryHidePrimaryView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView2[] = L"Windows.UI.ViewManagement.Core.ICoreInputView2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_XYFocusTransferringFromPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewTransferringXYFocusEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_XYFocusTransferringFromPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_XYFocusTransferredToPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_XYFocusTransferredToPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* TryTransferXYFocusToPrimaryView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2* This,
        struct __x_ABI_CWindows_CFoundation_CRect origin,
        enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewXYFocusTransferDirection direction,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_add_XYFocusTransferringFromPrimaryView(This, handler, token) \
    ((This)->lpVtbl->add_XYFocusTransferringFromPrimaryView(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_remove_XYFocusTransferringFromPrimaryView(This, token) \
    ((This)->lpVtbl->remove_XYFocusTransferringFromPrimaryView(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_add_XYFocusTransferredToPrimaryView(This, handler, token) \
    ((This)->lpVtbl->add_XYFocusTransferredToPrimaryView(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_remove_XYFocusTransferredToPrimaryView(This, token) \
    ((This)->lpVtbl->remove_XYFocusTransferredToPrimaryView(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_TryTransferXYFocusToPrimaryView(This, origin, direction, result) \
    ((This)->lpVtbl->TryTransferXYFocusToPrimaryView(This, origin, direction, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView3[] = L"Windows.UI.ViewManagement.Core.ICoreInputView3";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryShow)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryShowWithKind)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewKind type,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryHide)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_TryShow(This, result) \
    ((This)->lpVtbl->TryShow(This, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_TryShowWithKind(This, type, result) \
    ((This)->lpVtbl->TryShowWithKind(This, type, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_TryHide(This, result) \
    ((This)->lpVtbl->TryHide(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView4[] = L"Windows.UI.ViewManagement.Core.ICoreInputView4";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PrimaryViewShowing)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewShowingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrimaryViewShowing)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PrimaryViewHiding)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewHidingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrimaryViewHiding)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_add_PrimaryViewShowing(This, handler, token) \
    ((This)->lpVtbl->add_PrimaryViewShowing(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_remove_PrimaryViewShowing(This, token) \
    ((This)->lpVtbl->remove_PrimaryViewShowing(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_add_PrimaryViewHiding(This, handler, token) \
    ((This)->lpVtbl->add_PrimaryViewHiding(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_remove_PrimaryViewHiding(This, token) \
    ((This)->lpVtbl->remove_PrimaryViewHiding(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputView5[] = L"Windows.UI.ViewManagement.Core.ICoreInputView5";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsKindSupported)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewKind type,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_SupportedKindsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SupportedKindsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PrimaryViewAnimationStarting)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CCore__CCoreInputView_Windows__CUI__CViewManagement__CCore__CCoreInputViewAnimationStartingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrimaryViewAnimationStarting)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_IsKindSupported(This, type, result) \
    ((This)->lpVtbl->IsKindSupported(This, type, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_add_SupportedKindsChanged(This, handler, token) \
    ((This)->lpVtbl->add_SupportedKindsChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_remove_SupportedKindsChanged(This, token) \
    ((This)->lpVtbl->remove_SupportedKindsChanged(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_add_PrimaryViewAnimationStarting(This, handler, token) \
    ((This)->lpVtbl->add_PrimaryViewAnimationStarting(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_remove_PrimaryViewAnimationStarting(This, token) \
    ((This)->lpVtbl->remove_PrimaryViewAnimationStarting(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Occlusions)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AnimationDuration)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_get_Occlusions(This, value) \
    ((This)->lpVtbl->get_Occlusions(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_get_AnimationDuration(This, value) \
    ((This)->lpVtbl->get_AnimationDuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewAnimationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewHidingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCancel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_TryCancel(This, result) \
    ((This)->lpVtbl->TryCancel(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewHidingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewOcclusion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewOcclusion[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OccludingRect)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_OcclusionKind)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewOcclusionKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_get_OccludingRect(This, value) \
    ((This)->lpVtbl->get_OccludingRect(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_get_OcclusionKind(This, value) \
    ((This)->lpVtbl->get_OcclusionKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Occlusions)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        __FIVectorView_1_Windows__CUI__CViewManagement__CCore__CCoreInputViewOcclusion** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_get_Occlusions(This, value) \
    ((This)->lpVtbl->get_Occlusions(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewOcclusionsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewShowingEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCancel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_TryCancel(This, result) \
    ((This)->lpVtbl->TryCancel(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewStatics[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics* This,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewStatics2[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewStatics2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUIContext)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2* This,
        __x_ABI_CWindows_CUI_CIUIContext* context,
        __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputView** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_GetForUIContext(This, context, result) \
    ((This)->lpVtbl->GetForUIContext(This, context, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_ICoreInputViewTransferringXYFocusEventArgs[] = L"Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Origin)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_Direction)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CCore_CCoreInputViewXYFocusTransferDirection* value);
    HRESULT (STDMETHODCALLTYPE* put_TransferHandled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TransferHandled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeepPrimaryViewVisible)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeepPrimaryViewVisible)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_get_Origin(This, value) \
    ((This)->lpVtbl->get_Origin(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_get_Direction(This, value) \
    ((This)->lpVtbl->get_Direction(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_put_TransferHandled(This, value) \
    ((This)->lpVtbl->put_TransferHandled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_get_TransferHandled(This, value) \
    ((This)->lpVtbl->get_TransferHandled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_put_KeepPrimaryViewVisible(This, value) \
    ((This)->lpVtbl->put_KeepPrimaryViewVisible(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_get_KeepPrimaryViewVisible(This, value) \
    ((This)->lpVtbl->get_KeepPrimaryViewVisible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CICoreInputViewTransferringXYFocusEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.IUISettingsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.UISettingsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_IUISettingsController[] = L"Windows.UI.ViewManagement.Core.IUISettingsController";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetAdvancedEffectsEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetAnimationsEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetAutoHideScrollBars)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetMessageDuration)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* SetTextScaleFactor)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_SetAdvancedEffectsEnabled(This, value) \
    ((This)->lpVtbl->SetAdvancedEffectsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_SetAnimationsEnabled(This, value) \
    ((This)->lpVtbl->SetAnimationsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_SetAutoHideScrollBars(This, value) \
    ((This)->lpVtbl->SetAutoHideScrollBars(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_SetMessageDuration(This, value) \
    ((This)->lpVtbl->SetMessageDuration(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_SetTextScaleFactor(This, value) \
    ((This)->lpVtbl->SetTextScaleFactor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.Core.IUISettingsControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.Core.UISettingsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_Core_IUISettingsControllerStatics[] = L"Windows.UI.ViewManagement.Core.IUISettingsControllerStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestDefaultAsync)(__x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics* This,
        __FIAsyncOperation_1_Windows__CUI__CViewManagement__CCore__CUISettingsController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_RequestDefaultAsync(This, operation) \
    ((This)->lpVtbl->RequestDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CCore_CIUISettingsControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputView ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputView[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreFrameworkInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreInputViewStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.ICoreInputViewStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputView ** Default Interface **
 *    Windows.UI.ViewManagement.Core.ICoreInputView2
 *    Windows.UI.ViewManagement.Core.ICoreInputView3
 *    Windows.UI.ViewManagement.Core.ICoreInputView4
 *    Windows.UI.ViewManagement.Core.ICoreInputView5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputView[] = L"Windows.UI.ViewManagement.Core.CoreInputView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewAnimationStartingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewHidingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewOcclusion[] = L"Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewOcclusionsChangedEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewShowingEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_CoreInputViewTransferringXYFocusEventArgs[] = L"Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.ViewManagement.Core.UISettingsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.Core.IUISettingsControllerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.Core.IUISettingsController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_Core_UISettingsController_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_Core_UISettingsController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_Core_UISettingsController[] = L"Windows.UI.ViewManagement.Core.UISettingsController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Eviewmanagement2Ecore_p_h__

#endif // __windows2Eui2Eviewmanagement2Ecore_h__
