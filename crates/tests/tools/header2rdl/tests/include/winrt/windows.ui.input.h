
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
#ifndef __windows2Eui2Einput_h__
#define __windows2Eui2Einput_h__
#ifndef __windows2Eui2Einput_p_h__
#define __windows2Eui2Einput_p_h__


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
#include "Windows.Devices.Haptics.h"
#include "Windows.Devices.Input.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.UI.Core.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IAttachableInputObject;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject ABI::Windows::UI::Input::IAttachableInputObject

#endif // ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IAttachableInputObjectFactory;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory ABI::Windows::UI::Input::IAttachableInputObjectFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ICrossSlidingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs ABI::Windows::UI::Input::ICrossSlidingEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ICrossSlidingEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2 ABI::Windows::UI::Input::ICrossSlidingEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IDraggingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs ABI::Windows::UI::Input::IDraggingEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IDraggingEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2 ABI::Windows::UI::Input::IDraggingEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IEdgeGesture;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture ABI::Windows::UI::Input::IEdgeGesture

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IEdgeGestureEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs ABI::Windows::UI::Input::IEdgeGestureEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IEdgeGestureStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics ABI::Windows::UI::Input::IEdgeGestureStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IGestureRecognizer;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer ABI::Windows::UI::Input::IGestureRecognizer

#endif // ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IGestureRecognizer2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2 ABI::Windows::UI::Input::IGestureRecognizer2

#endif // ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IHoldingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs ABI::Windows::UI::Input::IHoldingEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IHoldingEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2 ABI::Windows::UI::Input::IHoldingEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IInputActivationListener;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener ABI::Windows::UI::Input::IInputActivationListener

#endif // ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IInputActivationListenerActivationChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs ABI::Windows::UI::Input::IInputActivationListenerActivationChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IKeyboardDeliveryInterceptor;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor ABI::Windows::UI::Input::IKeyboardDeliveryInterceptor

#endif // ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IKeyboardDeliveryInterceptorStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics ABI::Windows::UI::Input::IKeyboardDeliveryInterceptorStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationCompletedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs ABI::Windows::UI::Input::IManipulationCompletedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationCompletedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2 ABI::Windows::UI::Input::IManipulationCompletedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationInertiaStartingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs ABI::Windows::UI::Input::IManipulationInertiaStartingEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationInertiaStartingEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2 ABI::Windows::UI::Input::IManipulationInertiaStartingEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationStartedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs ABI::Windows::UI::Input::IManipulationStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationStartedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2 ABI::Windows::UI::Input::IManipulationStartedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationUpdatedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs ABI::Windows::UI::Input::IManipulationUpdatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IManipulationUpdatedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2 ABI::Windows::UI::Input::IManipulationUpdatedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IMouseWheelParameters;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters ABI::Windows::UI::Input::IMouseWheelParameters

#endif // ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPhysicalGestureRecognizer;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer ABI::Windows::UI::Input::IPhysicalGestureRecognizer

#endif // ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPointPhysicalPosition;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition ABI::Windows::UI::Input::IPointerPointPhysicalPosition

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPointProperties;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties ABI::Windows::UI::Input::IPointerPointProperties

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPointProperties2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2 ABI::Windows::UI::Input::IPointerPointProperties2

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPointStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics ABI::Windows::UI::Input::IPointerPointStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPointTransform;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform ABI::Windows::UI::Input::IPointerPointTransform

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerVisualizationSettings;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings ABI::Windows::UI::Input::IPointerVisualizationSettings

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerVisualizationSettingsStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics ABI::Windows::UI::Input::IPointerVisualizationSettingsStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialController ABI::Windows::UI::Input::IRadialController

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialController2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialController2 ABI::Windows::UI::Input::IRadialController2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerButtonClickedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs ABI::Windows::UI::Input::IRadialControllerButtonClickedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerButtonClickedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2 ABI::Windows::UI::Input::IRadialControllerButtonClickedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerButtonHoldingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs ABI::Windows::UI::Input::IRadialControllerButtonHoldingEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerButtonPressedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs ABI::Windows::UI::Input::IRadialControllerButtonPressedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerButtonReleasedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs ABI::Windows::UI::Input::IRadialControllerButtonReleasedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerConfiguration;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration ABI::Windows::UI::Input::IRadialControllerConfiguration

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerConfiguration2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2 ABI::Windows::UI::Input::IRadialControllerConfiguration2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerConfigurationStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics ABI::Windows::UI::Input::IRadialControllerConfigurationStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerConfigurationStatics2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2 ABI::Windows::UI::Input::IRadialControllerConfigurationStatics2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerControlAcquiredEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs ABI::Windows::UI::Input::IRadialControllerControlAcquiredEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerControlAcquiredEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2 ABI::Windows::UI::Input::IRadialControllerControlAcquiredEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerMenu;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu ABI::Windows::UI::Input::IRadialControllerMenu

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerMenuItem;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem ABI::Windows::UI::Input::IRadialControllerMenuItem

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerMenuItemStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics ABI::Windows::UI::Input::IRadialControllerMenuItemStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerMenuItemStatics2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2 ABI::Windows::UI::Input::IRadialControllerMenuItemStatics2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerRotationChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs ABI::Windows::UI::Input::IRadialControllerRotationChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerRotationChangedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2 ABI::Windows::UI::Input::IRadialControllerRotationChangedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContact;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact ABI::Windows::UI::Input::IRadialControllerScreenContact

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContactContinuedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs ABI::Windows::UI::Input::IRadialControllerScreenContactContinuedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContactContinuedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2 ABI::Windows::UI::Input::IRadialControllerScreenContactContinuedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContactEndedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs ABI::Windows::UI::Input::IRadialControllerScreenContactEndedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContactStartedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs ABI::Windows::UI::Input::IRadialControllerScreenContactStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerScreenContactStartedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2 ABI::Windows::UI::Input::IRadialControllerScreenContactStartedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialControllerStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics ABI::Windows::UI::Input::IRadialControllerStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRightTappedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs ABI::Windows::UI::Input::IRightTappedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRightTappedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2 ABI::Windows::UI::Input::IRightTappedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ISystemButtonEventController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController ABI::Windows::UI::Input::ISystemButtonEventController

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ISystemButtonEventControllerStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics ABI::Windows::UI::Input::ISystemButtonEventControllerStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ISystemFunctionButtonEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs ABI::Windows::UI::Input::ISystemFunctionButtonEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ISystemFunctionLockChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs ABI::Windows::UI::Input::ISystemFunctionLockChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ISystemFunctionLockIndicatorChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs ABI::Windows::UI::Input::ISystemFunctionLockIndicatorChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ITappedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs ABI::Windows::UI::Input::ITappedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ITappedEventArgs2;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2 ABI::Windows::UI::Input::ITappedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ITouchpadGesturesController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController ABI::Windows::UI::Input::ITouchpadGesturesController

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ITouchpadGesturesControllerStatics;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics ABI::Windows::UI::Input::ITouchpadGesturesControllerStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface ITouchpadGlobalActionEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs ABI::Windows::UI::Input::ITouchpadGlobalActionEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PointerPoint;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerMenuItem;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5a773e24-d968-535e-969a-76ce3602a637"))
IIterator<ABI::Windows::UI::Input::RadialControllerMenuItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerMenuItem*, ABI::Windows::UI::Input::IRadialControllerMenuItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.RadialControllerMenuItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::RadialControllerMenuItem*> __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_t;
#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1aa752b3-dc11-5bce-b2b9-cd1bf8f235be"))
IIterable<ABI::Windows::UI::Input::RadialControllerMenuItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerMenuItem*, ABI::Windows::UI::Input::IRadialControllerMenuItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.RadialControllerMenuItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::RadialControllerMenuItem*> __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_t;
#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum RadialControllerSystemMenuItemKind : int RadialControllerSystemMenuItemKind;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3fbc1858-a43e-54dc-b0e2-8b098bddacf6"))
IIterator<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind> : IIterator_impl<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.RadialControllerSystemMenuItemKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind> __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_t;
#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4516010f-fd98-5e1d-bf3f-aeaf79f1f3da"))
IIterable<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind> : IIterable_impl<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.RadialControllerSystemMenuItemKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind> __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_t;
#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b91a7342-3fcb-512b-aef1-fda4712e5817"))
IVectorView<ABI::Windows::UI::Input::RadialControllerMenuItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerMenuItem*, ABI::Windows::UI::Input::IRadialControllerMenuItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.RadialControllerMenuItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::RadialControllerMenuItem*> __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_t;
#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#define DEF___FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfde94e7-70f8-5cc0-98e2-8c0f8ce524ab"))
IVector<ABI::Windows::UI::Input::RadialControllerMenuItem*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerMenuItem*, ABI::Windows::UI::Input::IRadialControllerMenuItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Input.RadialControllerMenuItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Input::RadialControllerMenuItem*> __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_t;
#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000


#ifndef DEF___FIReference_1_float_USE
#define DEF___FIReference_1_float_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("719cc2ba-3e76-5def-9f1a-38d85a145ea8"))
IReference<float> : IReference_impl<float>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Single>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<float> __FIReference_1_float_t;
#define __FIReference_1_float ABI::Windows::Foundation::__FIReference_1_float_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_float_USE */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class EdgeGesture;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class EdgeGestureEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b62d7f11-c7b1-5bf0-a347-3366252db4ce"))
ITypedEventHandler<ABI::Windows::UI::Input::EdgeGesture*, ABI::Windows::UI::Input::EdgeGestureEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::EdgeGesture*, ABI::Windows::UI::Input::IEdgeGesture*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::EdgeGestureEventArgs*, ABI::Windows::UI::Input::IEdgeGestureEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.EdgeGesture, Windows.UI.Input.EdgeGestureEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::EdgeGesture*, ABI::Windows::UI::Input::EdgeGestureEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class GestureRecognizer;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class CrossSlidingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1fc1eb70-0c26-5cf0-9e50-843a34083613"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::CrossSlidingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::CrossSlidingEventArgs*, ABI::Windows::UI::Input::ICrossSlidingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.CrossSlidingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::CrossSlidingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class DraggingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("be7cca42-98dd-5d72-9e91-976113d7985e"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::DraggingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::DraggingEventArgs*, ABI::Windows::UI::Input::IDraggingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.DraggingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::DraggingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class HoldingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0027bdf5-6948-53f7-afbe-d4d8fe500feb"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::HoldingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::HoldingEventArgs*, ABI::Windows::UI::Input::IHoldingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.HoldingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::HoldingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class ManipulationCompletedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c24f5605-3059-52cc-bdf3-7098146d34d7"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationCompletedEventArgs*, ABI::Windows::UI::Input::IManipulationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.ManipulationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class ManipulationInertiaStartingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("350a72ab-34a5-5193-913d-3d4c5983842c"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationInertiaStartingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationInertiaStartingEventArgs*, ABI::Windows::UI::Input::IManipulationInertiaStartingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.ManipulationInertiaStartingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationInertiaStartingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class ManipulationStartedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("110d75c8-ba5e-5462-8b2d-4a1582574b61"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationStartedEventArgs*, ABI::Windows::UI::Input::IManipulationStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.ManipulationStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class ManipulationUpdatedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eb5216f5-020a-5e40-9c4b-48912c690e2d"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*, ABI::Windows::UI::Input::IManipulationUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.ManipulationUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RightTappedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8420d242-ba34-5b17-92f3-8f767f1d0de4"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::RightTappedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RightTappedEventArgs*, ABI::Windows::UI::Input::IRightTappedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.RightTappedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::RightTappedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class TappedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("27f3ad57-8994-5712-8b83-a93ba17c03c2"))
ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::TappedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::IGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::TappedEventArgs*, ABI::Windows::UI::Input::ITappedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.GestureRecognizer, Windows.UI.Input.TappedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::GestureRecognizer*, ABI::Windows::UI::Input::TappedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class InputActivationListener;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class InputActivationListenerActivationChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("27256541-ca03-5192-bb2e-8ef1daf0bdeb"))
ITypedEventHandler<ABI::Windows::UI::Input::InputActivationListener*, ABI::Windows::UI::Input::InputActivationListenerActivationChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::InputActivationListener*, ABI::Windows::UI::Input::IInputActivationListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::InputActivationListenerActivationChangedEventArgs*, ABI::Windows::UI::Input::IInputActivationListenerActivationChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.InputActivationListener, Windows.UI.Input.InputActivationListenerActivationChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::InputActivationListener*, ABI::Windows::UI::Input::InputActivationListenerActivationChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class KeyboardDeliveryInterceptor;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class KeyEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c1caf11-9d6f-5e0f-a9ea-4c7c734d73d6"))
ITypedEventHandler<ABI::Windows::UI::Input::KeyboardDeliveryInterceptor*, ABI::Windows::UI::Core::KeyEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::KeyboardDeliveryInterceptor*, ABI::Windows::UI::Input::IKeyboardDeliveryInterceptor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::KeyEventArgs*, ABI::Windows::UI::Core::IKeyEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.KeyboardDeliveryInterceptor, Windows.UI.Core.KeyEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::KeyboardDeliveryInterceptor*, ABI::Windows::UI::Core::KeyEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PhysicalGestureRecognizer;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("adaa5f37-ba59-5c13-83e1-811a8c1c26ee"))
ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::HoldingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::IPhysicalGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::HoldingEventArgs*, ABI::Windows::UI::Input::IHoldingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.PhysicalGestureRecognizer, Windows.UI.Input.HoldingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::HoldingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7577ec2c-6f2b-5de1-8dbb-205f02dc46e4"))
ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::IPhysicalGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationCompletedEventArgs*, ABI::Windows::UI::Input::IManipulationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.PhysicalGestureRecognizer, Windows.UI.Input.ManipulationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80fefaa0-0335-5b09-8fe1-33854fe7e8c0"))
ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::IPhysicalGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationStartedEventArgs*, ABI::Windows::UI::Input::IManipulationStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.PhysicalGestureRecognizer, Windows.UI.Input.ManipulationStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fd08d645-0934-5c84-a498-fe313030966d"))
ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::IPhysicalGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*, ABI::Windows::UI::Input::IManipulationUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.PhysicalGestureRecognizer, Windows.UI.Input.ManipulationUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::ManipulationUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dde6ff59-be05-528a-947c-da5db69f8e6a"))
ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::TappedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::IPhysicalGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::TappedEventArgs*, ABI::Windows::UI::Input::ITappedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.PhysicalGestureRecognizer, Windows.UI.Input.TappedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::PhysicalGestureRecognizer*, ABI::Windows::UI::Input::TappedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e0f93b1-c2f2-5351-82aa-6cf5f4c2d068"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerButtonClickedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d09d9fcc-edb8-56c0-856d-70e477a9ddf3"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonClickedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerButtonClickedEventArgs*, ABI::Windows::UI::Input::IRadialControllerButtonClickedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerButtonClickedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonClickedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerButtonHoldingEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4c44f2ff-3a4a-51ba-a01f-9f2002471f59"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonHoldingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerButtonHoldingEventArgs*, ABI::Windows::UI::Input::IRadialControllerButtonHoldingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerButtonHoldingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonHoldingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerButtonPressedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("660bee4a-4fed-5a62-aa5d-8113b477bc69"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonPressedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerButtonPressedEventArgs*, ABI::Windows::UI::Input::IRadialControllerButtonPressedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerButtonPressedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonPressedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerButtonReleasedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c22ff62c-c642-5d50-9340-fe163122720c"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonReleasedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerButtonReleasedEventArgs*, ABI::Windows::UI::Input::IRadialControllerButtonReleasedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerButtonReleasedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerButtonReleasedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerControlAcquiredEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4ec5f3fd-b217-5452-a2bd-9725ce9f6675"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerControlAcquiredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerControlAcquiredEventArgs*, ABI::Windows::UI::Input::IRadialControllerControlAcquiredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerControlAcquiredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerControlAcquiredEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerRotationChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8fe73ac9-8a36-5152-815d-03310ee8bf85"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerRotationChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerRotationChangedEventArgs*, ABI::Windows::UI::Input::IRadialControllerRotationChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerRotationChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerRotationChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerScreenContactContinuedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("30579e67-fb4f-5d38-83b4-9cb610090def"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerScreenContactContinuedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerScreenContactContinuedEventArgs*, ABI::Windows::UI::Input::IRadialControllerScreenContactContinuedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerScreenContactContinuedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerScreenContactStartedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1f2d584e-3ad8-5049-b451-3a44a102fa82"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerScreenContactStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::IRadialController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerScreenContactStartedEventArgs*, ABI::Windows::UI::Input::IRadialControllerScreenContactStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialController, Windows.UI.Input.RadialControllerScreenContactStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialController*, ABI::Windows::UI::Input::RadialControllerScreenContactStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5c90d6fb-e4f8-5be2-8544-9de33f82c41a"))
ITypedEventHandler<ABI::Windows::UI::Input::RadialControllerMenuItem*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::RadialControllerMenuItem*, ABI::Windows::UI::Input::IRadialControllerMenuItem*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.RadialControllerMenuItem, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::RadialControllerMenuItem*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class SystemButtonEventController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class SystemFunctionButtonEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b19c7b2a-a907-5297-8279-adba1140218d"))
ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionButtonEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::ISystemButtonEventController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemFunctionButtonEventArgs*, ABI::Windows::UI::Input::ISystemFunctionButtonEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.SystemButtonEventController, Windows.UI.Input.SystemFunctionButtonEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionButtonEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class SystemFunctionLockChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("52c14f10-ff66-5358-bbfe-022df20e79ea"))
ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionLockChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::ISystemButtonEventController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemFunctionLockChangedEventArgs*, ABI::Windows::UI::Input::ISystemFunctionLockChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.SystemButtonEventController, Windows.UI.Input.SystemFunctionLockChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionLockChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class SystemFunctionLockIndicatorChangedEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e0846333-0428-5440-93d7-1829d3ca674f"))
ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionLockIndicatorChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::ISystemButtonEventController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::SystemFunctionLockIndicatorChangedEventArgs*, ABI::Windows::UI::Input::ISystemFunctionLockIndicatorChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.SystemButtonEventController, Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::SystemButtonEventController*, ABI::Windows::UI::Input::SystemFunctionLockIndicatorChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class TouchpadGesturesController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class PointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

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

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0b05a28c-2064-5b06-94f3-324fb004ece0"))
ITypedEventHandler<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Input::ITouchpadGesturesController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.TouchpadGesturesController, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class TouchpadGlobalActionEventArgs;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3fdb14d0-c3b2-576e-b900-aaa14e0ff77c"))
ITypedEventHandler<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Input::TouchpadGlobalActionEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Input::ITouchpadGesturesController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::TouchpadGlobalActionEventArgs*, ABI::Windows::UI::Input::ITouchpadGlobalActionEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.TouchpadGesturesController, Windows.UI.Input.TouchpadGlobalActionEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::TouchpadGesturesController*, ABI::Windows::UI::Input::TouchpadGlobalActionEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class SimpleHapticsController;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface ISimpleHapticsController;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController ABI::Windows::Devices::Haptics::ISimpleHapticsController

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PointerDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPointerDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice ABI::Windows::Devices::Input::IPointerDevice

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                typedef enum PointerDeviceType : int PointerDeviceType;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class RandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
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
            namespace Input {
                typedef enum CrossSlidingState : int CrossSlidingState;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum DraggingState : int DraggingState;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum EdgeGestureKind : int EdgeGestureKind;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum GestureSettings : unsigned int GestureSettings;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum HoldingState : int HoldingState;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum InputActivationState : int InputActivationState;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum PointerUpdateKind : int PointerUpdateKind;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum RadialControllerMenuKnownIcon : int RadialControllerMenuKnownIcon;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum TouchpadGlobalAction : int TouchpadGlobalAction;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum TouchpadGlobalGestureKinds : unsigned int TouchpadGlobalGestureKinds;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef struct CrossSlideThresholds CrossSlideThresholds;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef struct ManipulationDelta ManipulationDelta;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef struct ManipulationVelocities ManipulationVelocities;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class MouseWheelParameters;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PointerPointProperties;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PointerVisualizationSettings;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerConfiguration;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerMenu;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialControllerScreenContact;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.CrossSlidingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum CrossSlidingState : int
                {
                    CrossSlidingState_Started = 0,
                    CrossSlidingState_Dragging = 1,
                    CrossSlidingState_Selecting = 2,
                    CrossSlidingState_SelectSpeedBumping = 3,
                    CrossSlidingState_SpeedBumping = 4,
                    CrossSlidingState_Rearranging = 5,
                    CrossSlidingState_Completed = 6,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.DraggingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum DraggingState : int
                {
                    DraggingState_Started = 0,
                    DraggingState_Continuing = 1,
                    DraggingState_Completed = 2,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.EdgeGestureKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum EdgeGestureKind : int
                {
                    EdgeGestureKind_Touch = 0,
                    EdgeGestureKind_Keyboard = 1,
                    EdgeGestureKind_Mouse = 2,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.GazeInputAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum GazeInputAccessStatus : int
                {
                    GazeInputAccessStatus_Unspecified = 0,
                    GazeInputAccessStatus_Allowed = 1,
                    GazeInputAccessStatus_DeniedByUser = 2,
                    GazeInputAccessStatus_DeniedBySystem = 3,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Input.GestureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum GestureSettings : unsigned int
                {
                    GestureSettings_None = 0,
                    GestureSettings_Tap = 0x1,
                    GestureSettings_DoubleTap = 0x2,
                    GestureSettings_Hold = 0x4,
                    GestureSettings_HoldWithMouse = 0x8,
                    GestureSettings_RightTap = 0x10,
                    GestureSettings_Drag = 0x20,
                    GestureSettings_ManipulationTranslateX = 0x40,
                    GestureSettings_ManipulationTranslateY = 0x80,
                    GestureSettings_ManipulationTranslateRailsX = 0x100,
                    GestureSettings_ManipulationTranslateRailsY = 0x200,
                    GestureSettings_ManipulationRotate = 0x400,
                    GestureSettings_ManipulationScale = 0x800,
                    GestureSettings_ManipulationTranslateInertia = 0x1000,
                    GestureSettings_ManipulationRotateInertia = 0x2000,
                    GestureSettings_ManipulationScaleInertia = 0x4000,
                    GestureSettings_CrossSlide = 0x8000,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    GestureSettings_ManipulationMultipleFingerPanning = 0x10000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };

                DEFINE_ENUM_FLAG_OPERATORS(GestureSettings)
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.HoldingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum HoldingState : int
                {
                    HoldingState_Started = 0,
                    HoldingState_Completed = 1,
                    HoldingState_Canceled = 2,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.InputActivationState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum InputActivationState : int
                {
                    InputActivationState_None = 0,
                    InputActivationState_Deactivated = 1,
                    InputActivationState_ActivatedNotForeground = 2,
                    InputActivationState_ActivatedInForeground = 3,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Input.PointerUpdateKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum PointerUpdateKind : int
                {
                    PointerUpdateKind_Other = 0,
                    PointerUpdateKind_LeftButtonPressed = 1,
                    PointerUpdateKind_LeftButtonReleased = 2,
                    PointerUpdateKind_RightButtonPressed = 3,
                    PointerUpdateKind_RightButtonReleased = 4,
                    PointerUpdateKind_MiddleButtonPressed = 5,
                    PointerUpdateKind_MiddleButtonReleased = 6,
                    PointerUpdateKind_XButton1Pressed = 7,
                    PointerUpdateKind_XButton1Released = 8,
                    PointerUpdateKind_XButton2Pressed = 9,
                    PointerUpdateKind_XButton2Released = 10,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.RadialControllerMenuKnownIcon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum RadialControllerMenuKnownIcon : int
                {
                    RadialControllerMenuKnownIcon_Scroll = 0,
                    RadialControllerMenuKnownIcon_Zoom = 1,
                    RadialControllerMenuKnownIcon_UndoRedo = 2,
                    RadialControllerMenuKnownIcon_Volume = 3,
                    RadialControllerMenuKnownIcon_NextPreviousTrack = 4,
                    RadialControllerMenuKnownIcon_Ruler = 5,
                    RadialControllerMenuKnownIcon_InkColor = 6,
                    RadialControllerMenuKnownIcon_InkThickness = 7,
                    RadialControllerMenuKnownIcon_PenType = 8,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.RadialControllerSystemMenuItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum RadialControllerSystemMenuItemKind : int
                {
                    RadialControllerSystemMenuItemKind_Scroll = 0,
                    RadialControllerSystemMenuItemKind_Zoom = 1,
                    RadialControllerSystemMenuItemKind_UndoRedo = 2,
                    RadialControllerSystemMenuItemKind_Volume = 3,
                    RadialControllerSystemMenuItemKind_NextPreviousTrack = 4,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.TouchpadGlobalAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum TouchpadGlobalAction : int
                {
                    TouchpadGlobalAction_ThreeFingerTap = 0,
                    TouchpadGlobalAction_FourFingerTap = 1,
                    TouchpadGlobalAction_FiveFingerTap = 2,
                    TouchpadGlobalAction_ThreeFingerPressDown = 3,
                    TouchpadGlobalAction_FourFingerPressDown = 4,
                    TouchpadGlobalAction_FiveFingerPressDown = 5,
                    TouchpadGlobalAction_ThreeFingerPressUp = 6,
                    TouchpadGlobalAction_FourFingerPressUp = 7,
                    TouchpadGlobalAction_FiveFingerPressUp = 8,
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.UI.Input.TouchpadGlobalGestureKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                enum TouchpadGlobalGestureKinds : unsigned int
                {
                    TouchpadGlobalGestureKinds_None = 0,
                    TouchpadGlobalGestureKinds_ThreeFingerManipulations = 0x1,
                    TouchpadGlobalGestureKinds_FourFingerManipulations = 0x2,
                    TouchpadGlobalGestureKinds_FiveFingerManipulations = 0x4,
                    TouchpadGlobalGestureKinds_ThreeFingerActions = 0x8,
                    TouchpadGlobalGestureKinds_FourFingerActions = 0x10,
                    TouchpadGlobalGestureKinds_FiveFingerActions = 0x20,
                };

                DEFINE_ENUM_FLAG_OPERATORS(TouchpadGlobalGestureKinds)
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.UI.Input.CrossSlideThresholds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                struct CrossSlideThresholds
                {
                    FLOAT SelectionStart;
                    FLOAT SpeedBumpStart;
                    FLOAT SpeedBumpEnd;
                    FLOAT RearrangeStart;
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.ManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                struct ManipulationDelta
                {
                    ABI::Windows::Foundation::Point Translation;
                    FLOAT Scale;
                    FLOAT Rotation;
                    FLOAT Expansion;
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.ManipulationVelocities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                struct ManipulationVelocities
                {
                    ABI::Windows::Foundation::Point Linear;
                    FLOAT Angular;
                    FLOAT Expansion;
                };
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IAttachableInputObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.AttachableInputObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IAttachableInputObject[] = L"Windows.UI.Input.IAttachableInputObject";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("9b822734-a3c1-542a-b2f4-0e32b773fb07")
                IAttachableInputObject : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IAttachableInputObject = __uuidof(IAttachableInputObject);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIAttachableInputObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IAttachableInputObjectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.AttachableInputObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IAttachableInputObjectFactory[] = L"Windows.UI.Input.IAttachableInputObjectFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("a4c54c4e-42bc-58fa-a640-ea1516f4c06b")
                IAttachableInputObjectFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IAttachableInputObjectFactory = __uuidof(IAttachableInputObjectFactory);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.ICrossSlidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.CrossSlidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ICrossSlidingEventArgs[] = L"Windows.UI.Input.ICrossSlidingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("e9374738-6f88-41d9-8720-78e08e398349")
                ICrossSlidingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CrossSlidingState(
                        ABI::Windows::UI::Input::CrossSlidingState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICrossSlidingEventArgs = __uuidof(ICrossSlidingEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.ICrossSlidingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.CrossSlidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ICrossSlidingEventArgs2[] = L"Windows.UI.Input.ICrossSlidingEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("eefb7d48-c070-59f3-8dab-bcaf621d8687")
                ICrossSlidingEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICrossSlidingEventArgs2 = __uuidof(ICrossSlidingEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IDraggingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.DraggingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IDraggingEventArgs[] = L"Windows.UI.Input.IDraggingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("1c905384-083c-4bd3-b559-179cddeb33ec")
                IDraggingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DraggingState(
                        ABI::Windows::UI::Input::DraggingState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDraggingEventArgs = __uuidof(IDraggingEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IDraggingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.DraggingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IDraggingEventArgs2[] = L"Windows.UI.Input.IDraggingEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("71efdbf9-382a-55ca-b4b9-008123c1bf1a")
                IDraggingEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDraggingEventArgs2 = __uuidof(IDraggingEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IEdgeGesture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGesture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGesture[] = L"Windows.UI.Input.IEdgeGesture";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("580d5292-2ab1-49aa-a7f0-33bd3f8df9f1")
                IEdgeGesture : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Starting(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Starting(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Completed(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Completed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Canceled(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Canceled(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEdgeGesture = __uuidof(IEdgeGesture);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGesture;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IEdgeGestureEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGestureEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGestureEventArgs[] = L"Windows.UI.Input.IEdgeGestureEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("44fa4a24-2d09-42e1-8b5e-368208796a4c")
                IEdgeGestureEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::UI::Input::EdgeGestureKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEdgeGestureEventArgs = __uuidof(IEdgeGestureEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IEdgeGestureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGesture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGestureStatics[] = L"Windows.UI.Input.IEdgeGestureStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("bc6a8519-18ee-4043-9839-4fc584d60a14")
                IEdgeGestureStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Input::IEdgeGesture** current
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEdgeGestureStatics = __uuidof(IEdgeGestureStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.GestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IGestureRecognizer[] = L"Windows.UI.Input.IGestureRecognizer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("b47a37bf-3d6b-4f88-83e8-6dcb4012ffb0")
                IGestureRecognizer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GestureSettings(
                        ABI::Windows::UI::Input::GestureSettings* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GestureSettings(
                        ABI::Windows::UI::Input::GestureSettings value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInertial(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShowGestureFeedback(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShowGestureFeedback(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PivotCenter(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PivotCenter(
                        ABI::Windows::Foundation::Point value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PivotRadius(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PivotRadius(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaTranslationDeceleration(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaTranslationDeceleration(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaRotationDeceleration(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaRotationDeceleration(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaExpansionDeceleration(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaExpansionDeceleration(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaTranslationDisplacement(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaTranslationDisplacement(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaRotationAngle(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaRotationAngle(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InertiaExpansion(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InertiaExpansion(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ManipulationExact(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ManipulationExact(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CrossSlideThresholds(
                        ABI::Windows::UI::Input::CrossSlideThresholds* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CrossSlideThresholds(
                        ABI::Windows::UI::Input::CrossSlideThresholds value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CrossSlideHorizontally(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CrossSlideHorizontally(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CrossSlideExact(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CrossSlideExact(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoProcessInertia(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoProcessInertia(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MouseWheelParameters(
                        ABI::Windows::UI::Input::IMouseWheelParameters** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CanBeDoubleTap(
                        ABI::Windows::UI::Input::IPointerPoint* value,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessDownEvent(
                        ABI::Windows::UI::Input::IPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessMoveEvents(
                        __FIVector_1_Windows__CUI__CInput__CPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessUpEvent(
                        ABI::Windows::UI::Input::IPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessMouseWheelEvent(
                        ABI::Windows::UI::Input::IPointerPoint* value,
                        boolean isShiftKeyDown,
                        boolean isControlKeyDown
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessInertia(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CompleteGesture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Tapped(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Tapped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RightTapped(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RightTapped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Holding(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Holding(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Dragging(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Dragging(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationStarted(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationStarted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationUpdated(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationInertiaStarting(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationInertiaStarting(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationCompleted(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CrossSliding(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CrossSliding(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGestureRecognizer = __uuidof(IGestureRecognizer);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IGestureRecognizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.GestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IGestureRecognizer2[] = L"Windows.UI.Input.IGestureRecognizer2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("d646097f-6ef7-5746-8ba8-8ff2206e6f3b")
                IGestureRecognizer2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TapMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TapMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TapMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TapMaxContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldMaxContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldRadius(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldRadius(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldStartDelay(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldStartDelay(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TranslationMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TranslationMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TranslationMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TranslationMaxContactCount(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGestureRecognizer2 = __uuidof(IGestureRecognizer2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.HoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IHoldingEventArgs[] = L"Windows.UI.Input.IHoldingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("2bf755c5-e799-41b4-bb40-242f40959b71")
                IHoldingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldingState(
                        ABI::Windows::UI::Input::HoldingState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHoldingEventArgs = __uuidof(IHoldingEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IHoldingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.HoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IHoldingEventArgs2[] = L"Windows.UI.Input.IHoldingEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("141da9ea-4c79-5674-afea-493fdeb91f19")
                IHoldingEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHoldingEventArgs2 = __uuidof(IHoldingEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IInputActivationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.InputActivationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IInputActivationListener[] = L"Windows.UI.Input.IInputActivationListener";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("5d6d4ed2-28c7-5ae3-aa74-c918a9f243ca")
                IInputActivationListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::UI::Input::InputActivationState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_InputActivationChanged(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InputActivationChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputActivationListener = __uuidof(IInputActivationListener);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIInputActivationListener;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.InputActivationListenerActivationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IInputActivationListenerActivationChangedEventArgs[] = L"Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("7699b465-1dcf-5791-b4b9-6cafbeed2056")
                IInputActivationListenerActivationChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::UI::Input::InputActivationState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputActivationListenerActivationChangedEventArgs = __uuidof(IInputActivationListenerActivationChangedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IKeyboardDeliveryInterceptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IKeyboardDeliveryInterceptor[] = L"Windows.UI.Input.IKeyboardDeliveryInterceptor";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("b4baf068-8f49-446c-8db5-8c0ffe85cc9e")
                IKeyboardDeliveryInterceptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsInterceptionEnabledWhenInForeground(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInterceptionEnabledWhenInForeground(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyDown(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyDown(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyUp(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyUp(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyboardDeliveryInterceptor = __uuidof(IKeyboardDeliveryInterceptor);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.IKeyboardDeliveryInterceptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IKeyboardDeliveryInterceptorStatics[] = L"Windows.UI.Input.IKeyboardDeliveryInterceptorStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("f9f63ba2-ceba-4755-8a7e-14c0ffecd239")
                IKeyboardDeliveryInterceptorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Input::IKeyboardDeliveryInterceptor** keyboardDeliverySettings
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyboardDeliveryInterceptorStatics = __uuidof(IKeyboardDeliveryInterceptorStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.IManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationCompletedEventArgs[] = L"Windows.UI.Input.IManipulationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("b34ab22b-d19b-46ff-9f38-dec7754bb9e7")
                IManipulationCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cumulative(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Velocities(
                        ABI::Windows::UI::Input::ManipulationVelocities* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationCompletedEventArgs = __uuidof(IManipulationCompletedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationCompletedEventArgs2[] = L"Windows.UI.Input.IManipulationCompletedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("f0c0dce7-30a9-5b96-886f-6560a85e4757")
                IManipulationCompletedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationCompletedEventArgs2 = __uuidof(IManipulationCompletedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationInertiaStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationInertiaStartingEventArgs[] = L"Windows.UI.Input.IManipulationInertiaStartingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("dd37a898-26bf-467a-9ce5-ccf3fb11371e")
                IManipulationInertiaStartingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delta(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cumulative(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Velocities(
                        ABI::Windows::UI::Input::ManipulationVelocities* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationInertiaStartingEventArgs = __uuidof(IManipulationInertiaStartingEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationInertiaStartingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationInertiaStartingEventArgs2[] = L"Windows.UI.Input.IManipulationInertiaStartingEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("c25409b8-f9fa-5a45-bd97-dcbbb2201860")
                IManipulationInertiaStartingEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationInertiaStartingEventArgs2 = __uuidof(IManipulationInertiaStartingEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationStartedEventArgs[] = L"Windows.UI.Input.IManipulationStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("ddec873e-cfce-4932-8c1d-3c3d011a34c0")
                IManipulationStartedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cumulative(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationStartedEventArgs = __uuidof(IManipulationStartedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationStartedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationStartedEventArgs2[] = L"Windows.UI.Input.IManipulationStartedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("2da3db4e-e583-5055-afaa-16fd986531a6")
                IManipulationStartedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationStartedEventArgs2 = __uuidof(IManipulationStartedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationUpdatedEventArgs[] = L"Windows.UI.Input.IManipulationUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("cb354ce5-abb8-4f9f-b3ce-8181aa61ad82")
                IManipulationUpdatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delta(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cumulative(
                        ABI::Windows::UI::Input::ManipulationDelta* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Velocities(
                        ABI::Windows::UI::Input::ManipulationVelocities* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationUpdatedEventArgs = __uuidof(IManipulationUpdatedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationUpdatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationUpdatedEventArgs2[] = L"Windows.UI.Input.IManipulationUpdatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("f3dfb96a-3306-5903-a1c5-ff9757a8689e")
                IManipulationUpdatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IManipulationUpdatedEventArgs2 = __uuidof(IManipulationUpdatedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IMouseWheelParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.MouseWheelParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IMouseWheelParameters[] = L"Windows.UI.Input.IMouseWheelParameters";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("ead0ca44-9ded-4037-8149-5e4cc2564468")
                IMouseWheelParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CharTranslation(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CharTranslation(
                        ABI::Windows::Foundation::Point value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeltaScale(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DeltaScale(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeltaRotationAngle(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DeltaRotationAngle(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageTranslation(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PageTranslation(
                        ABI::Windows::Foundation::Point value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMouseWheelParameters = __uuidof(IMouseWheelParameters);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPhysicalGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PhysicalGestureRecognizer
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPhysicalGestureRecognizer[] = L"Windows.UI.Input.IPhysicalGestureRecognizer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("79a29f4d-32a6-5aa5-a999-42b0b420c66d")
                IPhysicalGestureRecognizer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GestureSettings(
                        ABI::Windows::UI::Input::GestureSettings* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GestureSettings(
                        ABI::Windows::UI::Input::GestureSettings value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TapMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TapMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TapMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TapMaxContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldMaxContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldRadius(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldRadius(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HoldStartDelay(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HoldStartDelay(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TranslationMinContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TranslationMinContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TranslationMaxContactCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TranslationMaxContactCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessDownEvent(
                        ABI::Windows::UI::Input::IPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessMoveEvents(
                        __FIVector_1_Windows__CUI__CInput__CPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessUpEvent(
                        ABI::Windows::UI::Input::IPointerPoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CompleteGesture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationStarted(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationStarted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationUpdated(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ManipulationCompleted(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ManipulationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Tapped(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Tapped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Holding(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Holding(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhysicalGestureRecognizer = __uuidof(IPhysicalGestureRecognizer);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.IPointerPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPoint[] = L"Windows.UI.Input.IPointerPoint";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("e995317d-7296-42d9-8233-c5be73b74a4a")
                IPointerPoint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDevice(
                        ABI::Windows::Devices::Input::IPointerDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawPosition(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrameId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInContact(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::UI::Input::IPointerPointProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPoint = __uuidof(IPointerPoint);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPoint;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointPhysicalPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointPhysicalPosition[] = L"Windows.UI.Input.IPointerPointPhysicalPosition";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("003185a3-a5e7-4859-9c0b-89340204806c")
                IPointerPointPhysicalPosition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsPhysicalPositionSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhysicalPosition(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPointPhysicalPosition = __uuidof(IPointerPointPhysicalPosition);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.UI.Input.IPointerPointProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPointProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointProperties[] = L"Windows.UI.Input.IPointerPointProperties";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("c79d8a4b-c163-4ee7-803f-67ce79f9972d")
                IPointerPointProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Pressure(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInverted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEraser(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_XTilt(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_YTilt(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Twist(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactRectRaw(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TouchConfidence(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLeftButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRightButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMiddleButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MouseWheelDelta(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsHorizontalMouseWheel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPrimary(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInRange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBarrelButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsXButton1Pressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsXButton2Pressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerUpdateKind(
                        ABI::Windows::UI::Input::PointerUpdateKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE HasUsage(
                        UINT32 usagePage,
                        UINT32 usageId,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUsageValue(
                        UINT32 usagePage,
                        UINT32 usageId,
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPointProperties = __uuidof(IPointerPointProperties);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPointProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointProperties2[] = L"Windows.UI.Input.IPointerPointProperties2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("22c3433a-c83b-41c0-a296-5e232d64d6af")
                IPointerPointProperties2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ZDistance(
                        __FIReference_1_float** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPointProperties2 = __uuidof(IPointerPointProperties2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointStatics[] = L"Windows.UI.Input.IPointerPointStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("a506638d-2a1a-413e-bc75-9f38381cc069")
                IPointerPointStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentPoint(
                        UINT32 pointerId,
                        ABI::Windows::UI::Input::IPointerPoint** pointerPoint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetIntermediatePoints(
                        UINT32 pointerId,
                        __FIVector_1_Windows__CUI__CInput__CPointerPoint** pointerPoints
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentPointTransformed(
                        UINT32 pointerId,
                        ABI::Windows::UI::Input::IPointerPointTransform* transform,
                        ABI::Windows::UI::Input::IPointerPoint** pointerPoint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetIntermediatePointsTransformed(
                        UINT32 pointerId,
                        ABI::Windows::UI::Input::IPointerPointTransform* transform,
                        __FIVector_1_Windows__CUI__CInput__CPointerPoint** pointerPoints
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPointStatics = __uuidof(IPointerPointStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointTransform[] = L"Windows.UI.Input.IPointerPointTransform";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("4d5fe14f-b87c-4028-bc9c-59e9947fb056")
                IPointerPointTransform : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Inverse(
                        ABI::Windows::UI::Input::IPointerPointTransform** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryTransform(
                        ABI::Windows::Foundation::Point inPoint,
                        ABI::Windows::Foundation::Point* outPoint,
                        boolean* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransformBounds(
                        ABI::Windows::Foundation::Rect rect,
                        ABI::Windows::Foundation::Rect* returnValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerPointTransform = __uuidof(IPointerPointTransform);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointTransform;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerVisualizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerVisualizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerVisualizationSettings[] = L"Windows.UI.Input.IPointerVisualizationSettings";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("4d1e6461-84f7-499d-bd91-2a36e2b7aaa2")
                IPointerVisualizationSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsContactFeedbackEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsContactFeedbackEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsBarrelButtonFeedbackEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBarrelButtonFeedbackEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerVisualizationSettings = __uuidof(IPointerVisualizationSettings);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerVisualizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerVisualizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerVisualizationSettingsStatics[] = L"Windows.UI.Input.IPointerVisualizationSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("68870edb-165b-4214-b4f3-584eca8c8a69")
                IPointerVisualizationSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Input::IPointerVisualizationSettings** visualizationSettings
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerVisualizationSettingsStatics = __uuidof(IPointerVisualizationSettingsStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IRadialController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialController[] = L"Windows.UI.Input.IRadialController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3055d1c8-df51-43d4-b23b-0e1037467a09")
                IRadialController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Menu(
                        ABI::Windows::UI::Input::IRadialControllerMenu** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RotationResolutionInDegrees(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RotationResolutionInDegrees(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UseAutomaticHapticFeedback(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UseAutomaticHapticFeedback(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScreenContactStarted(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScreenContactStarted(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScreenContactEnded(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScreenContactEnded(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScreenContactContinued(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScreenContactContinued(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ControlLost(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ControlLost(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RotationChanged(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RotationChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ButtonClicked(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ButtonClicked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ControlAcquired(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ControlAcquired(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialController = __uuidof(IRadialController);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialController2[] = L"Windows.UI.Input.IRadialController2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577eff-4cee-11e6-b535-001bdc06ab3b")
                IRadialController2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ButtonPressed(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ButtonPressed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ButtonHolding(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ButtonHolding(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ButtonReleased(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ButtonReleased(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialController2 = __uuidof(IRadialController2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialController2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonClickedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonClickedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa438-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerButtonClickedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerButtonClickedEventArgs = __uuidof(IRadialControllerButtonClickedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonClickedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonClickedEventArgs2[] = L"Windows.UI.Input.IRadialControllerButtonClickedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef3-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerButtonClickedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerButtonClickedEventArgs2 = __uuidof(IRadialControllerButtonClickedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonHoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonHoldingEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonHoldingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577eee-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerButtonHoldingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerButtonHoldingEventArgs = __uuidof(IRadialControllerButtonHoldingEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonPressedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonPressedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577eed-4cee-11e6-b535-001bdc06ab3b")
                IRadialControllerButtonPressedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerButtonPressedEventArgs = __uuidof(IRadialControllerButtonPressedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonReleasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonReleasedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonReleasedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonReleasedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577eef-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerButtonReleasedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerButtonReleasedEventArgs = __uuidof(IRadialControllerButtonReleasedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfiguration[] = L"Windows.UI.Input.IRadialControllerConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("a6b79ecb-6a52-4430-910c-56370a9d6b42")
                IRadialControllerConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetDefaultMenuItems(
                        __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* buttons
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResetToDefaultMenuItems(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySelectDefaultMenuItem(
                        ABI::Windows::UI::Input::RadialControllerSystemMenuItemKind type,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerConfiguration = __uuidof(IRadialControllerConfiguration);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfiguration2[] = L"Windows.UI.Input.IRadialControllerConfiguration2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef7-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerConfiguration2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ActiveControllerWhenMenuIsSuppressed(
                        ABI::Windows::UI::Input::IRadialController* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActiveControllerWhenMenuIsSuppressed(
                        ABI::Windows::UI::Input::IRadialController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsMenuSuppressed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMenuSuppressed(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerConfiguration2 = __uuidof(IRadialControllerConfiguration2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfigurationStatics[] = L"Windows.UI.Input.IRadialControllerConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("79b6b0e5-069a-4486-a99d-8db772b9642f")
                IRadialControllerConfigurationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Input::IRadialControllerConfiguration** configuration
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerConfigurationStatics = __uuidof(IRadialControllerConfigurationStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfigurationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfigurationStatics2[] = L"Windows.UI.Input.IRadialControllerConfigurationStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("53e08b17-e205-48d3-9caf-80ff47c4d7c7")
                IRadialControllerConfigurationStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_AppController(
                        ABI::Windows::UI::Input::IRadialController* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppController(
                        ABI::Windows::UI::Input::IRadialController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsAppControllerEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAppControllerEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerConfigurationStatics2 = __uuidof(IRadialControllerConfigurationStatics2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerControlAcquiredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerControlAcquiredEventArgs[] = L"Windows.UI.Input.IRadialControllerControlAcquiredEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa439-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerControlAcquiredEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerControlAcquiredEventArgs = __uuidof(IRadialControllerControlAcquiredEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerControlAcquiredEventArgs2[] = L"Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef4-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerControlAcquiredEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerControlAcquiredEventArgs2 = __uuidof(IRadialControllerControlAcquiredEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenu
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenu[] = L"Windows.UI.Input.IRadialControllerMenu";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("8506b35d-f640-4412-aba0-bad077e5ea8a")
                IRadialControllerMenu : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Items(
                        __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSelectedMenuItem(
                        ABI::Windows::UI::Input::IRadialControllerMenuItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectMenuItem(
                        ABI::Windows::UI::Input::IRadialControllerMenuItem* menuItem
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySelectPreviouslySelectedMenuItem(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerMenu = __uuidof(IRadialControllerMenu);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItem[] = L"Windows.UI.Input.IRadialControllerMenuItem";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("c80fc98d-ad0b-4c9c-8f2f-136a2373a6ba")
                IRadialControllerMenuItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Invoked(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Invoked(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerMenuItem = __uuidof(IRadialControllerMenuItem);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItemStatics[] = L"Windows.UI.Input.IRadialControllerMenuItemStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("249e0887-d842-4524-9df8-e0d647edc887")
                IRadialControllerMenuItemStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIcon(
                        HSTRING displayText,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* icon,
                        ABI::Windows::UI::Input::IRadialControllerMenuItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromKnownIcon(
                        HSTRING displayText,
                        ABI::Windows::UI::Input::RadialControllerMenuKnownIcon value,
                        ABI::Windows::UI::Input::IRadialControllerMenuItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerMenuItemStatics = __uuidof(IRadialControllerMenuItemStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItemStatics2[] = L"Windows.UI.Input.IRadialControllerMenuItemStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("0cbb70be-7e3e-48bd-be04-2c7fcaa9c1ff")
                IRadialControllerMenuItemStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFontGlyph(
                        HSTRING displayText,
                        HSTRING glyph,
                        HSTRING fontFamily,
                        ABI::Windows::UI::Input::IRadialControllerMenuItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFontGlyphWithUri(
                        HSTRING displayText,
                        HSTRING glyph,
                        HSTRING fontFamily,
                        ABI::Windows::Foundation::IUriRuntimeClass* fontUri,
                        ABI::Windows::UI::Input::IRadialControllerMenuItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerMenuItemStatics2 = __uuidof(IRadialControllerMenuItemStatics2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerRotationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerRotationChangedEventArgs[] = L"Windows.UI.Input.IRadialControllerRotationChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa435-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerRotationChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RotationDeltaInDegrees(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerRotationChangedEventArgs = __uuidof(IRadialControllerRotationChangedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerRotationChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerRotationChangedEventArgs2[] = L"Windows.UI.Input.IRadialControllerRotationChangedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577eec-4cee-11e6-b535-001bdc06ab3b")
                IRadialControllerRotationChangedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerRotationChangedEventArgs2 = __uuidof(IRadialControllerRotationChangedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContact
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContact
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContact[] = L"Windows.UI.Input.IRadialControllerScreenContact";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa434-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerScreenContact : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Bounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContact = __uuidof(IRadialControllerScreenContact);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactContinuedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa437-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerScreenContactContinuedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContactContinuedEventArgs = __uuidof(IRadialControllerScreenContactContinuedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactContinuedEventArgs2[] = L"Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef1-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerScreenContactContinuedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContactContinuedEventArgs2 = __uuidof(IRadialControllerScreenContactContinuedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactEndedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactEndedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef2-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerScreenContactEndedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContactEndedEventArgs = __uuidof(IRadialControllerScreenContactEndedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactStartedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("206aa436-e651-11e5-bf62-2c27d7404e85")
                IRadialControllerScreenContactStartedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::UI::Input::IRadialControllerScreenContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContactStartedEventArgs = __uuidof(IRadialControllerScreenContactStartedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactStartedEventArgs2[] = L"Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("3d577ef0-3cee-11e6-b535-001bdc06ab3b")
                IRadialControllerScreenContactStartedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsButtonPressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerScreenContactStartedEventArgs2 = __uuidof(IRadialControllerScreenContactStartedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerStatics[] = L"Windows.UI.Input.IRadialControllerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("faded0b7-b84c-4894-87aa-8f25aa5f288b")
                IRadialControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForCurrentView(
                        ABI::Windows::UI::Input::IRadialController** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadialControllerStatics = __uuidof(IRadialControllerStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRightTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RightTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRightTappedEventArgs[] = L"Windows.UI.Input.IRightTappedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("4cbf40bd-af7a-4a36-9476-b1dce141709a")
                IRightTappedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRightTappedEventArgs = __uuidof(IRightTappedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IRightTappedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RightTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRightTappedEventArgs2[] = L"Windows.UI.Input.IRightTappedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("61c7b7bb-9f57-5857-a33c-c58c3dfa959e")
                IRightTappedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRightTappedEventArgs2 = __uuidof(IRightTappedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemButtonEventController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemButtonEventController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemButtonEventController[] = L"Windows.UI.Input.ISystemButtonEventController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("59b893a9-73bc-52b5-ba41-82511b2cb46c")
                ISystemButtonEventController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_SystemFunctionButtonPressed(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SystemFunctionButtonPressed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SystemFunctionButtonReleased(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SystemFunctionButtonReleased(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SystemFunctionLockChanged(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SystemFunctionLockChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SystemFunctionLockIndicatorChanged(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SystemFunctionLockIndicatorChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemButtonEventController = __uuidof(ISystemButtonEventController);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemButtonEventController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemButtonEventControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemButtonEventController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemButtonEventControllerStatics[] = L"Windows.UI.Input.ISystemButtonEventControllerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("632fb07b-20bd-5e15-af4a-00dbf2064ffa")
                ISystemButtonEventControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForDispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue* queue,
                        ABI::Windows::UI::Input::ISystemButtonEventController** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemButtonEventControllerStatics = __uuidof(ISystemButtonEventControllerStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionButtonEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionButtonEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionButtonEventArgs[] = L"Windows.UI.Input.ISystemFunctionButtonEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("4833896f-80d1-5dd6-92a7-62a508ffef5a")
                ISystemFunctionButtonEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemFunctionButtonEventArgs = __uuidof(ISystemFunctionButtonEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionLockChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionLockChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionLockChangedEventArgs[] = L"Windows.UI.Input.ISystemFunctionLockChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("cd040608-fcf9-585c-beab-f1d2eaf364ab")
                ISystemFunctionLockChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLocked(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemFunctionLockChangedEventArgs = __uuidof(ISystemFunctionLockChangedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionLockIndicatorChangedEventArgs[] = L"Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("b212b94e-7a6f-58ae-b304-bae61d0371b9")
                ISystemFunctionLockIndicatorChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIndicatorOn(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemFunctionLockIndicatorChangedEventArgs = __uuidof(ISystemFunctionLockIndicatorChangedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ITappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITappedEventArgs[] = L"Windows.UI.Input.ITappedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("cfa126e4-253a-4c3c-953b-395c37aed309")
                ITappedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TapCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITappedEventArgs = __uuidof(ITappedEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.ITappedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITappedEventArgs2[] = L"Windows.UI.Input.ITappedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("294388f2-177e-51d5-be56-ee0866fa968c")
                ITappedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITappedEventArgs2 = __uuidof(ITappedEventArgs2);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITappedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ITouchpadGesturesController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGesturesController
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGesturesController[] = L"Windows.UI.Input.ITouchpadGesturesController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("28c13cdd-e068-549f-89c6-1a440c6fc327")
                ITouchpadGesturesController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedGestures(
                        ABI::Windows::UI::Input::TouchpadGlobalGestureKinds* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportedGestures(
                        ABI::Windows::UI::Input::TouchpadGlobalGestureKinds value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerPressed(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerPressed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerMoved(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerMoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerReleased(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerReleased(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_GlobalActionPerformed(
                        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_GlobalActionPerformed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITouchpadGesturesController = __uuidof(ITouchpadGesturesController);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.ITouchpadGesturesControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGesturesController
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGesturesControllerStatics[] = L"Windows.UI.Input.ITouchpadGesturesControllerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("207ef171-1a73-51cd-a694-8840e09dbafa")
                ITouchpadGesturesControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForProcess(
                        ABI::Windows::UI::Input::ITouchpadGesturesController** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITouchpadGesturesControllerStatics = __uuidof(ITouchpadGesturesControllerStatics);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.ITouchpadGlobalActionEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGlobalActionEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGlobalActionEventArgs[] = L"Windows.UI.Input.ITouchpadGlobalActionEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                MIDL_INTERFACE("6edad206-e4e3-5f39-9d13-8575e8e2a12b")
                ITouchpadGlobalActionEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        ABI::Windows::UI::Input::TouchpadGlobalAction* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDevice(
                        ABI::Windows::Devices::Input::IPointerDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITouchpadGlobalActionEventArgs = __uuidof(ITouchpadGlobalActionEventArgs);
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.AttachableInputObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IAttachableInputObject ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_AttachableInputObject_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_AttachableInputObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_AttachableInputObject[] = L"Windows.UI.Input.AttachableInputObject";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.CrossSlidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ICrossSlidingEventArgs ** Default Interface **
 *    Windows.UI.Input.ICrossSlidingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_CrossSlidingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_CrossSlidingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_CrossSlidingEventArgs[] = L"Windows.UI.Input.CrossSlidingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.DraggingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IDraggingEventArgs ** Default Interface **
 *    Windows.UI.Input.IDraggingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_DraggingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_DraggingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_DraggingEventArgs[] = L"Windows.UI.Input.DraggingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.EdgeGesture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IEdgeGestureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IEdgeGesture ** Default Interface **
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_EdgeGesture_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_EdgeGesture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_EdgeGesture[] = L"Windows.UI.Input.EdgeGesture";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.EdgeGestureEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IEdgeGestureEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_EdgeGestureEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_EdgeGestureEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_EdgeGestureEventArgs[] = L"Windows.UI.Input.EdgeGestureEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.GestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IGestureRecognizer ** Default Interface **
 *    Windows.UI.Input.IGestureRecognizer2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_GestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_GestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_GestureRecognizer[] = L"Windows.UI.Input.GestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.HoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IHoldingEventArgs ** Default Interface **
 *    Windows.UI.Input.IHoldingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_HoldingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_HoldingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_HoldingEventArgs[] = L"Windows.UI.Input.HoldingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.InputActivationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IInputActivationListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_InputActivationListener_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_InputActivationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_InputActivationListener[] = L"Windows.UI.Input.InputActivationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.InputActivationListenerActivationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs[] = L"Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IKeyboardDeliveryInterceptorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IKeyboardDeliveryInterceptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_KeyboardDeliveryInterceptor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_KeyboardDeliveryInterceptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_KeyboardDeliveryInterceptor[] = L"Windows.UI.Input.KeyboardDeliveryInterceptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.ManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationCompletedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationCompletedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationCompletedEventArgs[] = L"Windows.UI.Input.ManipulationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationInertiaStartingEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationInertiaStartingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationInertiaStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationInertiaStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationInertiaStartingEventArgs[] = L"Windows.UI.Input.ManipulationInertiaStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationStartedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationStartedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationStartedEventArgs[] = L"Windows.UI.Input.ManipulationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationUpdatedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationUpdatedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationUpdatedEventArgs[] = L"Windows.UI.Input.ManipulationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.MouseWheelParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IMouseWheelParameters ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_MouseWheelParameters_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_MouseWheelParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_MouseWheelParameters[] = L"Windows.UI.Input.MouseWheelParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PhysicalGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPhysicalGestureRecognizer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_PhysicalGestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PhysicalGestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PhysicalGestureRecognizer[] = L"Windows.UI.Input.PhysicalGestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.PointerPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IPointerPointStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerPoint ** Default Interface **
 *    Windows.UI.Input.IPointerPointPhysicalPosition
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerPoint_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerPoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerPoint[] = L"Windows.UI.Input.PointerPoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PointerPointProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerPointProperties ** Default Interface **
 *    Windows.UI.Input.IPointerPointProperties2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerPointProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerPointProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerPointProperties[] = L"Windows.UI.Input.PointerPointProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PointerVisualizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IPointerVisualizationSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerVisualizationSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerVisualizationSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerVisualizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerVisualizationSettings[] = L"Windows.UI.Input.PointerVisualizationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.RadialController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialController ** Default Interface **
 *    Windows.UI.Input.IRadialController2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialController[] = L"Windows.UI.Input.RadialController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonClickedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerButtonClickedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonClickedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonHoldingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonHoldingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonHoldingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonHoldingEventArgs[] = L"Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonPressedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonReleasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonReleasedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonReleasedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonReleasedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonReleasedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerConfigurationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerConfigurationStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerConfiguration ** Default Interface **
 *    Windows.UI.Input.IRadialControllerConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerConfiguration[] = L"Windows.UI.Input.RadialControllerConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerControlAcquiredEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerControlAcquiredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerControlAcquiredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerControlAcquiredEventArgs[] = L"Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerMenu ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerMenu_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerMenu_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerMenu[] = L"Windows.UI.Input.RadialControllerMenu";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerMenuItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerMenuItemStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerMenuItemStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerMenuItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerMenuItem_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerMenuItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerMenuItem[] = L"Windows.UI.Input.RadialControllerMenuItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerRotationChangedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerRotationChangedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerRotationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerRotationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerRotationChangedEventArgs[] = L"Windows.UI.Input.RadialControllerRotationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContact
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContact ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContact_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContact_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContact[] = L"Windows.UI.Input.RadialControllerScreenContact";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RightTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRightTappedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRightTappedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_RightTappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RightTappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RightTappedEventArgs[] = L"Windows.UI.Input.RightTappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.SystemButtonEventController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.ISystemButtonEventControllerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemButtonEventController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemButtonEventController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemButtonEventController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemButtonEventController[] = L"Windows.UI.Input.SystemButtonEventController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionButtonEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionButtonEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionButtonEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionButtonEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionButtonEventArgs[] = L"Windows.UI.Input.SystemFunctionButtonEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionLockChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionLockChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionLockChangedEventArgs[] = L"Windows.UI.Input.SystemFunctionLockChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs[] = L"Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.TappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITappedEventArgs ** Default Interface **
 *    Windows.UI.Input.ITappedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_TappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TappedEventArgs[] = L"Windows.UI.Input.TappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.TouchpadGesturesController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.ITouchpadGesturesControllerStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITouchpadGesturesController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_TouchpadGesturesController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TouchpadGesturesController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TouchpadGesturesController[] = L"Windows.UI.Input.TouchpadGesturesController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.TouchpadGlobalActionEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITouchpadGlobalActionEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_TouchpadGlobalActionEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TouchpadGlobalActionEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TouchpadGlobalActionEventArgs[] = L"Windows.UI.Input.TouchpadGlobalActionEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject;

#endif // ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2 __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2 __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIEdgeGesture __x_ABI_CWindows_CUI_CInput_CIEdgeGesture;

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer;

#endif // ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2 __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2 __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIInputActivationListener __x_ABI_CWindows_CUI_CInput_CIInputActivationListener;

#endif // ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor;

#endif // ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2 __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters;

#endif // ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPoint __x_ABI_CWindows_CUI_CInput_CIPointerPoint;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2 __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialController __x_ABI_CWindows_CUI_CInput_CIRadialController;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialController2 __x_ABI_CWindows_CUI_CInput_CIRadialController2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2 __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController;

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CITappedEventArgs __x_ABI_CWindows_CUI_CInput_CITappedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2 __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController;

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem;

typedef struct __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem;

typedef struct __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __FIIterator_1_Windows__CUI__CInput__CRadialControllerMenuItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind;

typedef struct __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        enum __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind;

typedef struct __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* This,
        __FIIterator_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem;

typedef struct __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __FIVectorView_1_Windows__CUI__CInput__CRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl;

interface __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if !defined(____FIReference_1_float_INTERFACE_DEFINED__)
#define ____FIReference_1_float_INTERFACE_DEFINED__

typedef interface __FIReference_1_float __FIReference_1_float;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_float;

typedef struct __FIReference_1_floatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_float* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_float* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_float* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_float* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_float* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_float* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_float* This,
        FLOAT* result);

    END_INTERFACE
} __FIReference_1_floatVtbl;

interface __FIReference_1_float
{
    CONST_VTBL struct __FIReference_1_floatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_float_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_float_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_float_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_float_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_float_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_float_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_float_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_float_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIEdgeGesture* sender,
        __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CITappedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIInputActivationListener* sender,
        __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* sender,
        __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CITappedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* sender,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* sender,
        __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* sender,
        __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* sender,
        __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* sender,
        __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPointerDevice __x_ABI_CWindows_CDevices_CInput_CIPointerDevice;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType;

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CInput_CCrossSlidingState __x_ABI_CWindows_CUI_CInput_CCrossSlidingState;

typedef enum __x_ABI_CWindows_CUI_CInput_CDraggingState __x_ABI_CWindows_CUI_CInput_CDraggingState;

typedef enum __x_ABI_CWindows_CUI_CInput_CEdgeGestureKind __x_ABI_CWindows_CUI_CInput_CEdgeGestureKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CGestureSettings __x_ABI_CWindows_CUI_CInput_CGestureSettings;

typedef enum __x_ABI_CWindows_CUI_CInput_CHoldingState __x_ABI_CWindows_CUI_CInput_CHoldingState;

typedef enum __x_ABI_CWindows_CUI_CInput_CInputActivationState __x_ABI_CWindows_CUI_CInput_CInputActivationState;

typedef enum __x_ABI_CWindows_CUI_CInput_CPointerUpdateKind __x_ABI_CWindows_CUI_CInput_CPointerUpdateKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CRadialControllerMenuKnownIcon __x_ABI_CWindows_CUI_CInput_CRadialControllerMenuKnownIcon;

typedef enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalAction __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalAction;

typedef enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalGestureKinds __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalGestureKinds;

typedef struct __x_ABI_CWindows_CUI_CInput_CCrossSlideThresholds __x_ABI_CWindows_CUI_CInput_CCrossSlideThresholds;

typedef struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta __x_ABI_CWindows_CUI_CInput_CManipulationDelta;

typedef struct __x_ABI_CWindows_CUI_CInput_CManipulationVelocities __x_ABI_CWindows_CUI_CInput_CManipulationVelocities;

/*
 *
 * Struct Windows.UI.Input.CrossSlidingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CCrossSlidingState
{
    CrossSlidingState_Started = 0,
    CrossSlidingState_Dragging = 1,
    CrossSlidingState_Selecting = 2,
    CrossSlidingState_SelectSpeedBumping = 3,
    CrossSlidingState_SpeedBumping = 4,
    CrossSlidingState_Rearranging = 5,
    CrossSlidingState_Completed = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.DraggingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CDraggingState
{
    DraggingState_Started = 0,
    DraggingState_Continuing = 1,
    DraggingState_Completed = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.EdgeGestureKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CEdgeGestureKind
{
    EdgeGestureKind_Touch = 0,
    EdgeGestureKind_Keyboard = 1,
    EdgeGestureKind_Mouse = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.GazeInputAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CInput_CGazeInputAccessStatus
{
    GazeInputAccessStatus_Unspecified = 0,
    GazeInputAccessStatus_Allowed = 1,
    GazeInputAccessStatus_DeniedByUser = 2,
    GazeInputAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Input.GestureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CGestureSettings
{
    GestureSettings_None = 0,
    GestureSettings_Tap = 0x1,
    GestureSettings_DoubleTap = 0x2,
    GestureSettings_Hold = 0x4,
    GestureSettings_HoldWithMouse = 0x8,
    GestureSettings_RightTap = 0x10,
    GestureSettings_Drag = 0x20,
    GestureSettings_ManipulationTranslateX = 0x40,
    GestureSettings_ManipulationTranslateY = 0x80,
    GestureSettings_ManipulationTranslateRailsX = 0x100,
    GestureSettings_ManipulationTranslateRailsY = 0x200,
    GestureSettings_ManipulationRotate = 0x400,
    GestureSettings_ManipulationScale = 0x800,
    GestureSettings_ManipulationTranslateInertia = 0x1000,
    GestureSettings_ManipulationRotateInertia = 0x2000,
    GestureSettings_ManipulationScaleInertia = 0x4000,
    GestureSettings_CrossSlide = 0x8000,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    GestureSettings_ManipulationMultipleFingerPanning = 0x10000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.HoldingState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CHoldingState
{
    HoldingState_Started = 0,
    HoldingState_Completed = 1,
    HoldingState_Canceled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.InputActivationState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CInput_CInputActivationState
{
    InputActivationState_None = 0,
    InputActivationState_Deactivated = 1,
    InputActivationState_ActivatedNotForeground = 2,
    InputActivationState_ActivatedInForeground = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Input.PointerUpdateKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPointerUpdateKind
{
    PointerUpdateKind_Other = 0,
    PointerUpdateKind_LeftButtonPressed = 1,
    PointerUpdateKind_LeftButtonReleased = 2,
    PointerUpdateKind_RightButtonPressed = 3,
    PointerUpdateKind_RightButtonReleased = 4,
    PointerUpdateKind_MiddleButtonPressed = 5,
    PointerUpdateKind_MiddleButtonReleased = 6,
    PointerUpdateKind_XButton1Pressed = 7,
    PointerUpdateKind_XButton1Released = 8,
    PointerUpdateKind_XButton2Pressed = 9,
    PointerUpdateKind_XButton2Released = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.RadialControllerMenuKnownIcon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CRadialControllerMenuKnownIcon
{
    RadialControllerMenuKnownIcon_Scroll = 0,
    RadialControllerMenuKnownIcon_Zoom = 1,
    RadialControllerMenuKnownIcon_UndoRedo = 2,
    RadialControllerMenuKnownIcon_Volume = 3,
    RadialControllerMenuKnownIcon_NextPreviousTrack = 4,
    RadialControllerMenuKnownIcon_Ruler = 5,
    RadialControllerMenuKnownIcon_InkColor = 6,
    RadialControllerMenuKnownIcon_InkThickness = 7,
    RadialControllerMenuKnownIcon_PenType = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.RadialControllerSystemMenuItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind
{
    RadialControllerSystemMenuItemKind_Scroll = 0,
    RadialControllerSystemMenuItemKind_Zoom = 1,
    RadialControllerSystemMenuItemKind_UndoRedo = 2,
    RadialControllerSystemMenuItemKind_Volume = 3,
    RadialControllerSystemMenuItemKind_NextPreviousTrack = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.TouchpadGlobalAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalAction
{
    TouchpadGlobalAction_ThreeFingerTap = 0,
    TouchpadGlobalAction_FourFingerTap = 1,
    TouchpadGlobalAction_FiveFingerTap = 2,
    TouchpadGlobalAction_ThreeFingerPressDown = 3,
    TouchpadGlobalAction_FourFingerPressDown = 4,
    TouchpadGlobalAction_FiveFingerPressDown = 5,
    TouchpadGlobalAction_ThreeFingerPressUp = 6,
    TouchpadGlobalAction_FourFingerPressUp = 7,
    TouchpadGlobalAction_FiveFingerPressUp = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.UI.Input.TouchpadGlobalGestureKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalGestureKinds
{
    TouchpadGlobalGestureKinds_None = 0,
    TouchpadGlobalGestureKinds_ThreeFingerManipulations = 0x1,
    TouchpadGlobalGestureKinds_FourFingerManipulations = 0x2,
    TouchpadGlobalGestureKinds_FiveFingerManipulations = 0x4,
    TouchpadGlobalGestureKinds_ThreeFingerActions = 0x8,
    TouchpadGlobalGestureKinds_FourFingerActions = 0x10,
    TouchpadGlobalGestureKinds_FiveFingerActions = 0x20,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.UI.Input.CrossSlideThresholds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CCrossSlideThresholds
{
    FLOAT SelectionStart;
    FLOAT SpeedBumpStart;
    FLOAT SpeedBumpEnd;
    FLOAT RearrangeStart;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.ManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta
{
    struct __x_ABI_CWindows_CFoundation_CPoint Translation;
    FLOAT Scale;
    FLOAT Rotation;
    FLOAT Expansion;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.ManipulationVelocities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CManipulationVelocities
{
    struct __x_ABI_CWindows_CFoundation_CPoint Linear;
    FLOAT Angular;
    FLOAT Expansion;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IAttachableInputObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.AttachableInputObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IAttachableInputObject[] = L"Windows.UI.Input.IAttachableInputObject";
typedef struct __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObject* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIAttachableInputObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IAttachableInputObjectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.AttachableInputObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IAttachableInputObjectFactory[] = L"Windows.UI.Input.IAttachableInputObjectFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIAttachableInputObjectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.ICrossSlidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.CrossSlidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ICrossSlidingEventArgs[] = L"Windows.UI.Input.ICrossSlidingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_CrossSlidingState)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CCrossSlidingState* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_get_CrossSlidingState(This, value) \
    ((This)->lpVtbl->get_CrossSlidingState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.ICrossSlidingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.CrossSlidingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ICrossSlidingEventArgs2[] = L"Windows.UI.Input.ICrossSlidingEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CICrossSlidingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IDraggingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.DraggingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IDraggingEventArgs[] = L"Windows.UI.Input.IDraggingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_DraggingState)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CDraggingState* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_get_DraggingState(This, value) \
    ((This)->lpVtbl->get_DraggingState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IDraggingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.DraggingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IDraggingEventArgs2[] = L"Windows.UI.Input.IDraggingEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIDraggingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IEdgeGesture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGesture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGesture[] = L"Windows.UI.Input.IEdgeGesture";
typedef struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Starting)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Starting)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Completed)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Completed)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Canceled)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CEdgeGesture_Windows__CUI__CInput__CEdgeGestureEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Canceled)(__x_ABI_CWindows_CUI_CInput_CIEdgeGesture* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIEdgeGestureVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIEdgeGesture
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_add_Starting(This, handler, token) \
    ((This)->lpVtbl->add_Starting(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_remove_Starting(This, token) \
    ((This)->lpVtbl->remove_Starting(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_add_Completed(This, handler, token) \
    ((This)->lpVtbl->add_Completed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_remove_Completed(This, token) \
    ((This)->lpVtbl->remove_Completed(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_add_Canceled(This, handler, token) \
    ((This)->lpVtbl->add_Canceled(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGesture_remove_Canceled(This, token) \
    ((This)->lpVtbl->remove_Canceled(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGesture;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGesture_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IEdgeGestureEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGestureEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGestureEventArgs[] = L"Windows.UI.Input.IEdgeGestureEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CEdgeGestureKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IEdgeGestureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.EdgeGesture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IEdgeGestureStatics[] = L"Windows.UI.Input.IEdgeGestureStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics* This,
        __x_ABI_CWindows_CUI_CInput_CIEdgeGesture** current);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_GetForCurrentView(This, current) \
    ((This)->lpVtbl->GetForCurrentView(This, current))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIEdgeGestureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.GestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IGestureRecognizer[] = L"Windows.UI.Input.IGestureRecognizer";
typedef struct __x_ABI_CWindows_CUI_CInput_CIGestureRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GestureSettings)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CGestureSettings* value);
    HRESULT (STDMETHODCALLTYPE* put_GestureSettings)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CGestureSettings value);
    HRESULT (STDMETHODCALLTYPE* get_IsInertial)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ShowGestureFeedback)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowGestureFeedback)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PivotCenter)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* put_PivotCenter)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* get_PivotRadius)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PivotRadius)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaTranslationDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaTranslationDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaRotationDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaRotationDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaExpansionDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaExpansionDeceleration)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaTranslationDisplacement)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaTranslationDisplacement)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaRotationAngle)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaRotationAngle)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_InertiaExpansion)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_InertiaExpansion)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_ManipulationExact)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ManipulationExact)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CrossSlideThresholds)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        struct __x_ABI_CWindows_CUI_CInput_CCrossSlideThresholds* value);
    HRESULT (STDMETHODCALLTYPE* put_CrossSlideThresholds)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        struct __x_ABI_CWindows_CUI_CInput_CCrossSlideThresholds value);
    HRESULT (STDMETHODCALLTYPE* get_CrossSlideHorizontally)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CrossSlideHorizontally)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CrossSlideExact)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CrossSlideExact)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AutoProcessInertia)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoProcessInertia)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MouseWheelParameters)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters** value);
    HRESULT (STDMETHODCALLTYPE* CanBeDoubleTap)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* ProcessDownEvent)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* ProcessMoveEvents)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FIVector_1_Windows__CUI__CInput__CPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* ProcessUpEvent)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* ProcessMouseWheelEvent)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value,
        boolean isShiftKeyDown,
        boolean isControlKeyDown);
    HRESULT (STDMETHODCALLTYPE* ProcessInertia)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* CompleteGesture)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* add_Tapped)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Tapped)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RightTapped)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CRightTappedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RightTapped)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Holding)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Holding)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Dragging)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CDraggingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Dragging)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationInertiaStarting)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationInertiaStartingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationInertiaStarting)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CrossSliding)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CGestureRecognizer_Windows__CUI__CInput__CCrossSlidingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CrossSliding)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIGestureRecognizerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIGestureRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_GestureSettings(This, value) \
    ((This)->lpVtbl->get_GestureSettings(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_GestureSettings(This, value) \
    ((This)->lpVtbl->put_GestureSettings(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_IsInertial(This, value) \
    ((This)->lpVtbl->get_IsInertial(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_ShowGestureFeedback(This, value) \
    ((This)->lpVtbl->get_ShowGestureFeedback(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_ShowGestureFeedback(This, value) \
    ((This)->lpVtbl->put_ShowGestureFeedback(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_PivotCenter(This, value) \
    ((This)->lpVtbl->get_PivotCenter(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_PivotCenter(This, value) \
    ((This)->lpVtbl->put_PivotCenter(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_PivotRadius(This, value) \
    ((This)->lpVtbl->get_PivotRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_PivotRadius(This, value) \
    ((This)->lpVtbl->put_PivotRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaTranslationDeceleration(This, value) \
    ((This)->lpVtbl->get_InertiaTranslationDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaTranslationDeceleration(This, value) \
    ((This)->lpVtbl->put_InertiaTranslationDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaRotationDeceleration(This, value) \
    ((This)->lpVtbl->get_InertiaRotationDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaRotationDeceleration(This, value) \
    ((This)->lpVtbl->put_InertiaRotationDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaExpansionDeceleration(This, value) \
    ((This)->lpVtbl->get_InertiaExpansionDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaExpansionDeceleration(This, value) \
    ((This)->lpVtbl->put_InertiaExpansionDeceleration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaTranslationDisplacement(This, value) \
    ((This)->lpVtbl->get_InertiaTranslationDisplacement(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaTranslationDisplacement(This, value) \
    ((This)->lpVtbl->put_InertiaTranslationDisplacement(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaRotationAngle(This, value) \
    ((This)->lpVtbl->get_InertiaRotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaRotationAngle(This, value) \
    ((This)->lpVtbl->put_InertiaRotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_InertiaExpansion(This, value) \
    ((This)->lpVtbl->get_InertiaExpansion(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_InertiaExpansion(This, value) \
    ((This)->lpVtbl->put_InertiaExpansion(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_ManipulationExact(This, value) \
    ((This)->lpVtbl->get_ManipulationExact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_ManipulationExact(This, value) \
    ((This)->lpVtbl->put_ManipulationExact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_CrossSlideThresholds(This, value) \
    ((This)->lpVtbl->get_CrossSlideThresholds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_CrossSlideThresholds(This, value) \
    ((This)->lpVtbl->put_CrossSlideThresholds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_CrossSlideHorizontally(This, value) \
    ((This)->lpVtbl->get_CrossSlideHorizontally(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_CrossSlideHorizontally(This, value) \
    ((This)->lpVtbl->put_CrossSlideHorizontally(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_CrossSlideExact(This, value) \
    ((This)->lpVtbl->get_CrossSlideExact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_CrossSlideExact(This, value) \
    ((This)->lpVtbl->put_CrossSlideExact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_AutoProcessInertia(This, value) \
    ((This)->lpVtbl->get_AutoProcessInertia(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_put_AutoProcessInertia(This, value) \
    ((This)->lpVtbl->put_AutoProcessInertia(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_get_MouseWheelParameters(This, value) \
    ((This)->lpVtbl->get_MouseWheelParameters(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_CanBeDoubleTap(This, value, result) \
    ((This)->lpVtbl->CanBeDoubleTap(This, value, result))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_ProcessDownEvent(This, value) \
    ((This)->lpVtbl->ProcessDownEvent(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_ProcessMoveEvents(This, value) \
    ((This)->lpVtbl->ProcessMoveEvents(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_ProcessUpEvent(This, value) \
    ((This)->lpVtbl->ProcessUpEvent(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_ProcessMouseWheelEvent(This, value, isShiftKeyDown, isControlKeyDown) \
    ((This)->lpVtbl->ProcessMouseWheelEvent(This, value, isShiftKeyDown, isControlKeyDown))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_ProcessInertia(This) \
    ((This)->lpVtbl->ProcessInertia(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_CompleteGesture(This) \
    ((This)->lpVtbl->CompleteGesture(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_Tapped(This, handler, token) \
    ((This)->lpVtbl->add_Tapped(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_Tapped(This, token) \
    ((This)->lpVtbl->remove_Tapped(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_RightTapped(This, handler, token) \
    ((This)->lpVtbl->add_RightTapped(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_RightTapped(This, token) \
    ((This)->lpVtbl->remove_RightTapped(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_Holding(This, handler, token) \
    ((This)->lpVtbl->add_Holding(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_Holding(This, token) \
    ((This)->lpVtbl->remove_Holding(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_Dragging(This, handler, token) \
    ((This)->lpVtbl->add_Dragging(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_Dragging(This, token) \
    ((This)->lpVtbl->remove_Dragging(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_ManipulationStarted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_ManipulationStarted(This, token) \
    ((This)->lpVtbl->remove_ManipulationStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_ManipulationUpdated(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationUpdated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_ManipulationUpdated(This, token) \
    ((This)->lpVtbl->remove_ManipulationUpdated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_ManipulationInertiaStarting(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationInertiaStarting(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_ManipulationInertiaStarting(This, token) \
    ((This)->lpVtbl->remove_ManipulationInertiaStarting(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_ManipulationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationCompleted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_ManipulationCompleted(This, token) \
    ((This)->lpVtbl->remove_ManipulationCompleted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_add_CrossSliding(This, handler, token) \
    ((This)->lpVtbl->add_CrossSliding(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_remove_CrossSliding(This, token) \
    ((This)->lpVtbl->remove_CrossSliding(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IGestureRecognizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.GestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IGestureRecognizer2[] = L"Windows.UI.Input.IGestureRecognizer2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TapMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TapMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TapMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TapMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldRadius)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldRadius)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_HoldStartDelay)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldStartDelay)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TranslationMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TranslationMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TranslationMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TranslationMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_TapMinContactCount(This, value) \
    ((This)->lpVtbl->get_TapMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_TapMinContactCount(This, value) \
    ((This)->lpVtbl->put_TapMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_TapMaxContactCount(This, value) \
    ((This)->lpVtbl->get_TapMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_TapMaxContactCount(This, value) \
    ((This)->lpVtbl->put_TapMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_HoldMinContactCount(This, value) \
    ((This)->lpVtbl->get_HoldMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_HoldMinContactCount(This, value) \
    ((This)->lpVtbl->put_HoldMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_HoldMaxContactCount(This, value) \
    ((This)->lpVtbl->get_HoldMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_HoldMaxContactCount(This, value) \
    ((This)->lpVtbl->put_HoldMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_HoldRadius(This, value) \
    ((This)->lpVtbl->get_HoldRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_HoldRadius(This, value) \
    ((This)->lpVtbl->put_HoldRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_HoldStartDelay(This, value) \
    ((This)->lpVtbl->get_HoldStartDelay(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_HoldStartDelay(This, value) \
    ((This)->lpVtbl->put_HoldStartDelay(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_TranslationMinContactCount(This, value) \
    ((This)->lpVtbl->get_TranslationMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_TranslationMinContactCount(This, value) \
    ((This)->lpVtbl->put_TranslationMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_get_TranslationMaxContactCount(This, value) \
    ((This)->lpVtbl->get_TranslationMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_put_TranslationMaxContactCount(This, value) \
    ((This)->lpVtbl->put_TranslationMaxContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIGestureRecognizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.HoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IHoldingEventArgs[] = L"Windows.UI.Input.IHoldingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_HoldingState)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CHoldingState* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_get_HoldingState(This, value) \
    ((This)->lpVtbl->get_HoldingState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IHoldingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.HoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IHoldingEventArgs2[] = L"Windows.UI.Input.IHoldingEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentContactCount)(__x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_get_CurrentContactCount(This, value) \
    ((This)->lpVtbl->get_CurrentContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIHoldingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IInputActivationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.InputActivationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IInputActivationListener[] = L"Windows.UI.Input.IInputActivationListener";
typedef struct __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        enum __x_ABI_CWindows_CUI_CInput_CInputActivationState* value);
    HRESULT (STDMETHODCALLTYPE* add_InputActivationChanged)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInputActivationListener_Windows__CUI__CInput__CInputActivationListenerActivationChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InputActivationChanged)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListener* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIInputActivationListener
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_add_InputActivationChanged(This, handler, token) \
    ((This)->lpVtbl->add_InputActivationChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListener_remove_InputActivationChanged(This, token) \
    ((This)->lpVtbl->remove_InputActivationChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIInputActivationListener;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.InputActivationListenerActivationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IInputActivationListenerActivationChangedEventArgs[] = L"Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CInputActivationState* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIInputActivationListenerActivationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.IKeyboardDeliveryInterceptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IKeyboardDeliveryInterceptor[] = L"Windows.UI.Input.IKeyboardDeliveryInterceptor";
typedef struct __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsInterceptionEnabledWhenInForeground)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInterceptionEnabledWhenInForeground)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_KeyDown)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_KeyDown)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_KeyUp)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CKeyboardDeliveryInterceptor_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_KeyUp)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_get_IsInterceptionEnabledWhenInForeground(This, value) \
    ((This)->lpVtbl->get_IsInterceptionEnabledWhenInForeground(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_put_IsInterceptionEnabledWhenInForeground(This, value) \
    ((This)->lpVtbl->put_IsInterceptionEnabledWhenInForeground(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_add_KeyDown(This, handler, token) \
    ((This)->lpVtbl->add_KeyDown(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_remove_KeyDown(This, token) \
    ((This)->lpVtbl->remove_KeyDown(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_add_KeyUp(This, handler, token) \
    ((This)->lpVtbl->add_KeyUp(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_remove_KeyUp(This, token) \
    ((This)->lpVtbl->remove_KeyUp(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.IKeyboardDeliveryInterceptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IKeyboardDeliveryInterceptorStatics[] = L"Windows.UI.Input.IKeyboardDeliveryInterceptorStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics* This,
        __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptor** keyboardDeliverySettings);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_GetForCurrentView(This, keyboardDeliverySettings) \
    ((This)->lpVtbl->GetForCurrentView(This, keyboardDeliverySettings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIKeyboardDeliveryInterceptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.IManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationCompletedEventArgs[] = L"Windows.UI.Input.IManipulationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Cumulative)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);
    HRESULT (STDMETHODCALLTYPE* get_Velocities)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationVelocities* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_get_Cumulative(This, value) \
    ((This)->lpVtbl->get_Cumulative(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_get_Velocities(This, value) \
    ((This)->lpVtbl->get_Velocities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationCompletedEventArgs2[] = L"Windows.UI.Input.IManipulationCompletedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_get_CurrentContactCount(This, value) \
    ((This)->lpVtbl->get_CurrentContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationInertiaStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationInertiaStartingEventArgs[] = L"Windows.UI.Input.IManipulationInertiaStartingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Delta)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);
    HRESULT (STDMETHODCALLTYPE* get_Cumulative)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);
    HRESULT (STDMETHODCALLTYPE* get_Velocities)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationVelocities* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_get_Delta(This, value) \
    ((This)->lpVtbl->get_Delta(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_get_Cumulative(This, value) \
    ((This)->lpVtbl->get_Cumulative(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_get_Velocities(This, value) \
    ((This)->lpVtbl->get_Velocities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationInertiaStartingEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationInertiaStartingEventArgs2[] = L"Windows.UI.Input.IManipulationInertiaStartingEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationInertiaStartingEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationStartedEventArgs[] = L"Windows.UI.Input.IManipulationStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Cumulative)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_get_Cumulative(This, value) \
    ((This)->lpVtbl->get_Cumulative(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationStartedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationStartedEventArgs2[] = L"Windows.UI.Input.IManipulationStartedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationStartedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationUpdatedEventArgs[] = L"Windows.UI.Input.IManipulationUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Delta)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);
    HRESULT (STDMETHODCALLTYPE* get_Cumulative)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationDelta* value);
    HRESULT (STDMETHODCALLTYPE* get_Velocities)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CManipulationVelocities* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_get_Delta(This, value) \
    ((This)->lpVtbl->get_Delta(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_get_Cumulative(This, value) \
    ((This)->lpVtbl->get_Cumulative(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_get_Velocities(This, value) \
    ((This)->lpVtbl->get_Velocities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IManipulationUpdatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IManipulationUpdatedEventArgs2[] = L"Windows.UI.Input.IManipulationUpdatedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentContactCount)(__x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_get_CurrentContactCount(This, value) \
    ((This)->lpVtbl->get_CurrentContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIManipulationUpdatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.IMouseWheelParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.MouseWheelParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IMouseWheelParameters[] = L"Windows.UI.Input.IMouseWheelParameters";
typedef struct __x_ABI_CWindows_CUI_CInput_CIMouseWheelParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CharTranslation)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* put_CharTranslation)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* get_DeltaScale)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_DeltaScale)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_DeltaRotationAngle)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_DeltaRotationAngle)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_PageTranslation)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* put_PageTranslation)(__x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIMouseWheelParametersVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIMouseWheelParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_get_CharTranslation(This, value) \
    ((This)->lpVtbl->get_CharTranslation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_put_CharTranslation(This, value) \
    ((This)->lpVtbl->put_CharTranslation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_get_DeltaScale(This, value) \
    ((This)->lpVtbl->get_DeltaScale(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_put_DeltaScale(This, value) \
    ((This)->lpVtbl->put_DeltaScale(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_get_DeltaRotationAngle(This, value) \
    ((This)->lpVtbl->get_DeltaRotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_put_DeltaRotationAngle(This, value) \
    ((This)->lpVtbl->put_DeltaRotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_get_PageTranslation(This, value) \
    ((This)->lpVtbl->get_PageTranslation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_put_PageTranslation(This, value) \
    ((This)->lpVtbl->put_PageTranslation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIMouseWheelParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPhysicalGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PhysicalGestureRecognizer
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPhysicalGestureRecognizer[] = L"Windows.UI.Input.IPhysicalGestureRecognizer";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_GestureSettings)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CGestureSettings* value);
    HRESULT (STDMETHODCALLTYPE* put_GestureSettings)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CGestureSettings value);
    HRESULT (STDMETHODCALLTYPE* get_TapMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TapMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TapMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TapMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HoldRadius)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldRadius)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_HoldStartDelay)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_HoldStartDelay)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TranslationMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TranslationMinContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TranslationMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TranslationMaxContactCount)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* ProcessDownEvent)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* ProcessMoveEvents)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FIVector_1_Windows__CUI__CInput__CPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* ProcessUpEvent)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* CompleteGesture)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CManipulationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Tapped)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CTappedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Tapped)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Holding)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPhysicalGestureRecognizer_Windows__CUI__CInput__CHoldingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Holding)(__x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_GestureSettings(This, value) \
    ((This)->lpVtbl->get_GestureSettings(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_GestureSettings(This, value) \
    ((This)->lpVtbl->put_GestureSettings(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_TapMinContactCount(This, value) \
    ((This)->lpVtbl->get_TapMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_TapMinContactCount(This, value) \
    ((This)->lpVtbl->put_TapMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_TapMaxContactCount(This, value) \
    ((This)->lpVtbl->get_TapMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_TapMaxContactCount(This, value) \
    ((This)->lpVtbl->put_TapMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_HoldMinContactCount(This, value) \
    ((This)->lpVtbl->get_HoldMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_HoldMinContactCount(This, value) \
    ((This)->lpVtbl->put_HoldMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_HoldMaxContactCount(This, value) \
    ((This)->lpVtbl->get_HoldMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_HoldMaxContactCount(This, value) \
    ((This)->lpVtbl->put_HoldMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_HoldRadius(This, value) \
    ((This)->lpVtbl->get_HoldRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_HoldRadius(This, value) \
    ((This)->lpVtbl->put_HoldRadius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_HoldStartDelay(This, value) \
    ((This)->lpVtbl->get_HoldStartDelay(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_HoldStartDelay(This, value) \
    ((This)->lpVtbl->put_HoldStartDelay(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_TranslationMinContactCount(This, value) \
    ((This)->lpVtbl->get_TranslationMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_TranslationMinContactCount(This, value) \
    ((This)->lpVtbl->put_TranslationMinContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_get_TranslationMaxContactCount(This, value) \
    ((This)->lpVtbl->get_TranslationMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_put_TranslationMaxContactCount(This, value) \
    ((This)->lpVtbl->put_TranslationMaxContactCount(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_ProcessDownEvent(This, value) \
    ((This)->lpVtbl->ProcessDownEvent(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_ProcessMoveEvents(This, value) \
    ((This)->lpVtbl->ProcessMoveEvents(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_ProcessUpEvent(This, value) \
    ((This)->lpVtbl->ProcessUpEvent(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_CompleteGesture(This) \
    ((This)->lpVtbl->CompleteGesture(This))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_add_ManipulationStarted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_remove_ManipulationStarted(This, token) \
    ((This)->lpVtbl->remove_ManipulationStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_add_ManipulationUpdated(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationUpdated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_remove_ManipulationUpdated(This, token) \
    ((This)->lpVtbl->remove_ManipulationUpdated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_add_ManipulationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationCompleted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_remove_ManipulationCompleted(This, token) \
    ((This)->lpVtbl->remove_ManipulationCompleted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_add_Tapped(This, handler, token) \
    ((This)->lpVtbl->add_Tapped(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_remove_Tapped(This, token) \
    ((This)->lpVtbl->remove_Tapped(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_add_Holding(This, handler, token) \
    ((This)->lpVtbl->add_Holding(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_remove_Holding(This, token) \
    ((This)->lpVtbl->remove_Holding(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPhysicalGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.IPointerPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPoint[] = L"Windows.UI.Input.IPointerPoint";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDevice)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_RawPosition)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerId)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FrameId)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInContact)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CUI_CInput_CIPointerPoint* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPoint
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_PointerDevice(This, value) \
    ((This)->lpVtbl->get_PointerDevice(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_RawPosition(This, value) \
    ((This)->lpVtbl->get_RawPosition(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_PointerId(This, value) \
    ((This)->lpVtbl->get_PointerId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_FrameId(This, value) \
    ((This)->lpVtbl->get_FrameId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_IsInContact(This, value) \
    ((This)->lpVtbl->get_IsInContact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPoint;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointPhysicalPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointPhysicalPosition[] = L"Windows.UI.Input.IPointerPointPhysicalPosition";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPhysicalPositionSupported)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalPosition)(__x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPositionVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_get_IsPhysicalPositionSupported(This, value) \
    ((This)->lpVtbl->get_IsPhysicalPositionSupported(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_get_PhysicalPosition(This, value) \
    ((This)->lpVtbl->get_PhysicalPosition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointPhysicalPosition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.UI.Input.IPointerPointProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPointProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointProperties[] = L"Windows.UI.Input.IPointerPointProperties";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Pressure)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInverted)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEraser)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_XTilt)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_YTilt)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Twist)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_ContactRect)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_ContactRectRaw)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_TouchConfidence)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLeftButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsRightButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMiddleButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MouseWheelDelta)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHorizontalMouseWheel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPrimary)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInRange)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBarrelButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsXButton1Pressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsXButton2Pressed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerUpdateKind)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        enum __x_ABI_CWindows_CUI_CInput_CPointerUpdateKind* value);
    HRESULT (STDMETHODCALLTYPE* HasUsage)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        UINT32 usagePage,
        UINT32 usageId,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetUsageValue)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties* This,
        UINT32 usagePage,
        UINT32 usageId,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointPropertiesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_Pressure(This, value) \
    ((This)->lpVtbl->get_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsInverted(This, value) \
    ((This)->lpVtbl->get_IsInverted(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsEraser(This, value) \
    ((This)->lpVtbl->get_IsEraser(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_XTilt(This, value) \
    ((This)->lpVtbl->get_XTilt(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_YTilt(This, value) \
    ((This)->lpVtbl->get_YTilt(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_Twist(This, value) \
    ((This)->lpVtbl->get_Twist(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_ContactRect(This, value) \
    ((This)->lpVtbl->get_ContactRect(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_ContactRectRaw(This, value) \
    ((This)->lpVtbl->get_ContactRectRaw(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_TouchConfidence(This, value) \
    ((This)->lpVtbl->get_TouchConfidence(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsLeftButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsLeftButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsRightButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsRightButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsMiddleButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsMiddleButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_MouseWheelDelta(This, value) \
    ((This)->lpVtbl->get_MouseWheelDelta(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsHorizontalMouseWheel(This, value) \
    ((This)->lpVtbl->get_IsHorizontalMouseWheel(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsPrimary(This, value) \
    ((This)->lpVtbl->get_IsPrimary(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsInRange(This, value) \
    ((This)->lpVtbl->get_IsInRange(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsBarrelButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsBarrelButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsXButton1Pressed(This, value) \
    ((This)->lpVtbl->get_IsXButton1Pressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_IsXButton2Pressed(This, value) \
    ((This)->lpVtbl->get_IsXButton2Pressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_get_PointerUpdateKind(This, value) \
    ((This)->lpVtbl->get_PointerUpdateKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_HasUsage(This, usagePage, usageId, value) \
    ((This)->lpVtbl->HasUsage(This, usagePage, usageId, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_GetUsageValue(This, usagePage, usageId, value) \
    ((This)->lpVtbl->GetUsageValue(This, usagePage, usageId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPointProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointProperties2[] = L"Windows.UI.Input.IPointerPointProperties2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ZDistance)(__x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2* This,
        __FIReference_1_float** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_get_ZDistance(This, value) \
    ((This)->lpVtbl->get_ZDistance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointStatics[] = L"Windows.UI.Input.IPointerPointStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentPoint)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        UINT32 pointerId,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** pointerPoint);
    HRESULT (STDMETHODCALLTYPE* GetIntermediatePoints)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        UINT32 pointerId,
        __FIVector_1_Windows__CUI__CInput__CPointerPoint** pointerPoints);
    HRESULT (STDMETHODCALLTYPE* GetCurrentPointTransformed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        UINT32 pointerId,
        __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* transform,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** pointerPoint);
    HRESULT (STDMETHODCALLTYPE* GetIntermediatePointsTransformed)(__x_ABI_CWindows_CUI_CInput_CIPointerPointStatics* This,
        UINT32 pointerId,
        __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* transform,
        __FIVector_1_Windows__CUI__CInput__CPointerPoint** pointerPoints);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetCurrentPoint(This, pointerId, pointerPoint) \
    ((This)->lpVtbl->GetCurrentPoint(This, pointerId, pointerPoint))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetIntermediatePoints(This, pointerId, pointerPoints) \
    ((This)->lpVtbl->GetIntermediatePoints(This, pointerId, pointerPoints))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetCurrentPointTransformed(This, pointerId, transform, pointerPoint) \
    ((This)->lpVtbl->GetCurrentPointTransformed(This, pointerId, transform, pointerPoint))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_GetIntermediatePointsTransformed(This, pointerId, transform, pointerPoints) \
    ((This)->lpVtbl->GetIntermediatePointsTransformed(This, pointerId, transform, pointerPoints))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerPointTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerPointTransform[] = L"Windows.UI.Input.IPointerPointTransform";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerPointTransformVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Inverse)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform** value);
    HRESULT (STDMETHODCALLTYPE* TryTransform)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        struct __x_ABI_CWindows_CFoundation_CPoint inPoint,
        struct __x_ABI_CWindows_CFoundation_CPoint* outPoint,
        boolean* returnValue);
    HRESULT (STDMETHODCALLTYPE* TransformBounds)(__x_ABI_CWindows_CUI_CInput_CIPointerPointTransform* This,
        struct __x_ABI_CWindows_CFoundation_CRect rect,
        struct __x_ABI_CWindows_CFoundation_CRect* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerPointTransformVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerPointTransformVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_get_Inverse(This, value) \
    ((This)->lpVtbl->get_Inverse(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_TryTransform(This, inPoint, outPoint, returnValue) \
    ((This)->lpVtbl->TryTransform(This, inPoint, outPoint, returnValue))

#define __x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_TransformBounds(This, rect, returnValue) \
    ((This)->lpVtbl->TransformBounds(This, rect, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerPointTransform;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerPointTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerVisualizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerVisualizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerVisualizationSettings[] = L"Windows.UI.Input.IPointerVisualizationSettings";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsContactFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsContactFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsBarrelButtonFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsBarrelButtonFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_put_IsContactFeedbackEnabled(This, value) \
    ((This)->lpVtbl->put_IsContactFeedbackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_get_IsContactFeedbackEnabled(This, value) \
    ((This)->lpVtbl->get_IsContactFeedbackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_put_IsBarrelButtonFeedbackEnabled(This, value) \
    ((This)->lpVtbl->put_IsBarrelButtonFeedbackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_get_IsBarrelButtonFeedbackEnabled(This, value) \
    ((This)->lpVtbl->get_IsBarrelButtonFeedbackEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IPointerVisualizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.PointerVisualizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IPointerVisualizationSettingsStatics[] = L"Windows.UI.Input.IPointerVisualizationSettingsStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettings** visualizationSettings);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_GetForCurrentView(This, visualizationSettings) \
    ((This)->lpVtbl->GetForCurrentView(This, visualizationSettings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIPointerVisualizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IRadialController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialController[] = L"Windows.UI.Input.IRadialController";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Menu)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu** value);
    HRESULT (STDMETHODCALLTYPE* get_RotationResolutionInDegrees)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RotationResolutionInDegrees)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_UseAutomaticHapticFeedback)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseAutomaticHapticFeedback)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_ScreenContactStarted)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactStartedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ScreenContactStarted)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_ScreenContactEnded)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ScreenContactEnded)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_ScreenContactContinued)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerScreenContactContinuedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ScreenContactContinued)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_ControlLost)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ControlLost)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_RotationChanged)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerRotationChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RotationChanged)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ButtonClicked)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonClickedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ButtonClicked)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ControlAcquired)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerControlAcquiredEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ControlAcquired)(__x_ABI_CWindows_CUI_CInput_CIRadialController* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_get_Menu(This, value) \
    ((This)->lpVtbl->get_Menu(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_get_RotationResolutionInDegrees(This, value) \
    ((This)->lpVtbl->get_RotationResolutionInDegrees(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_put_RotationResolutionInDegrees(This, value) \
    ((This)->lpVtbl->put_RotationResolutionInDegrees(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_get_UseAutomaticHapticFeedback(This, value) \
    ((This)->lpVtbl->get_UseAutomaticHapticFeedback(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_put_UseAutomaticHapticFeedback(This, value) \
    ((This)->lpVtbl->put_UseAutomaticHapticFeedback(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ScreenContactStarted(This, handler, cookie) \
    ((This)->lpVtbl->add_ScreenContactStarted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ScreenContactStarted(This, cookie) \
    ((This)->lpVtbl->remove_ScreenContactStarted(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ScreenContactEnded(This, handler, cookie) \
    ((This)->lpVtbl->add_ScreenContactEnded(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ScreenContactEnded(This, cookie) \
    ((This)->lpVtbl->remove_ScreenContactEnded(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ScreenContactContinued(This, handler, cookie) \
    ((This)->lpVtbl->add_ScreenContactContinued(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ScreenContactContinued(This, cookie) \
    ((This)->lpVtbl->remove_ScreenContactContinued(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ControlLost(This, handler, cookie) \
    ((This)->lpVtbl->add_ControlLost(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ControlLost(This, cookie) \
    ((This)->lpVtbl->remove_ControlLost(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_RotationChanged(This, handler, token) \
    ((This)->lpVtbl->add_RotationChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_RotationChanged(This, token) \
    ((This)->lpVtbl->remove_RotationChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ButtonClicked(This, handler, token) \
    ((This)->lpVtbl->add_ButtonClicked(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ButtonClicked(This, token) \
    ((This)->lpVtbl->remove_ButtonClicked(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_add_ControlAcquired(This, handler, cookie) \
    ((This)->lpVtbl->add_ControlAcquired(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController_remove_ControlAcquired(This, cookie) \
    ((This)->lpVtbl->remove_ControlAcquired(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialController2[] = L"Windows.UI.Input.IRadialController2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonPressedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ButtonHolding)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonHoldingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ButtonHolding)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ButtonReleased)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialController_Windows__CUI__CInput__CRadialControllerButtonReleasedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ButtonReleased)(__x_ABI_CWindows_CUI_CInput_CIRadialController2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialController2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialController2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_add_ButtonPressed(This, handler, token) \
    ((This)->lpVtbl->add_ButtonPressed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_remove_ButtonPressed(This, token) \
    ((This)->lpVtbl->remove_ButtonPressed(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_add_ButtonHolding(This, handler, token) \
    ((This)->lpVtbl->add_ButtonHolding(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_remove_ButtonHolding(This, token) \
    ((This)->lpVtbl->remove_ButtonHolding(This, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_add_ButtonReleased(This, handler, token) \
    ((This)->lpVtbl->add_ButtonReleased(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialController2_remove_ButtonReleased(This, token) \
    ((This)->lpVtbl->remove_ButtonReleased(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialController2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonClickedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonClickedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonClickedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonClickedEventArgs2[] = L"Windows.UI.Input.IRadialControllerButtonClickedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonClickedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonHoldingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonHoldingEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonHoldingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonHoldingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonPressedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonPressedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerButtonReleasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerButtonReleasedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerButtonReleasedEventArgs[] = L"Windows.UI.Input.IRadialControllerButtonReleasedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerButtonReleasedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfiguration[] = L"Windows.UI.Input.IRadialControllerConfiguration";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetDefaultMenuItems)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        __FIIterable_1_Windows__CUI__CInput__CRadialControllerSystemMenuItemKind* buttons);
    HRESULT (STDMETHODCALLTYPE* ResetToDefaultMenuItems)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* TrySelectDefaultMenuItem)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration* This,
        enum __x_ABI_CWindows_CUI_CInput_CRadialControllerSystemMenuItemKind type,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_SetDefaultMenuItems(This, buttons) \
    ((This)->lpVtbl->SetDefaultMenuItems(This, buttons))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_ResetToDefaultMenuItems(This) \
    ((This)->lpVtbl->ResetToDefaultMenuItems(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_TrySelectDefaultMenuItem(This, type, result) \
    ((This)->lpVtbl->TrySelectDefaultMenuItem(This, type, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfiguration2[] = L"Windows.UI.Input.IRadialControllerConfiguration2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ActiveControllerWhenMenuIsSuppressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* value);
    HRESULT (STDMETHODCALLTYPE* get_ActiveControllerWhenMenuIsSuppressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController** value);
    HRESULT (STDMETHODCALLTYPE* put_IsMenuSuppressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsMenuSuppressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_put_ActiveControllerWhenMenuIsSuppressed(This, value) \
    ((This)->lpVtbl->put_ActiveControllerWhenMenuIsSuppressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_get_ActiveControllerWhenMenuIsSuppressed(This, value) \
    ((This)->lpVtbl->get_ActiveControllerWhenMenuIsSuppressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_put_IsMenuSuppressed(This, value) \
    ((This)->lpVtbl->put_IsMenuSuppressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_get_IsMenuSuppressed(This, value) \
    ((This)->lpVtbl->get_IsMenuSuppressed(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfigurationStatics[] = L"Windows.UI.Input.IRadialControllerConfigurationStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfiguration** configuration);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_GetForCurrentView(This, configuration) \
    ((This)->lpVtbl->GetForCurrentView(This, configuration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerConfigurationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerConfigurationStatics2[] = L"Windows.UI.Input.IRadialControllerConfigurationStatics2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_AppController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController* value);
    HRESULT (STDMETHODCALLTYPE* get_AppController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController** value);
    HRESULT (STDMETHODCALLTYPE* put_IsAppControllerEnabled)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsAppControllerEnabled)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_put_AppController(This, value) \
    ((This)->lpVtbl->put_AppController(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_get_AppController(This, value) \
    ((This)->lpVtbl->get_AppController(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_put_IsAppControllerEnabled(This, value) \
    ((This)->lpVtbl->put_IsAppControllerEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_get_IsAppControllerEnabled(This, value) \
    ((This)->lpVtbl->get_IsAppControllerEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerConfigurationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerControlAcquiredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerControlAcquiredEventArgs[] = L"Windows.UI.Input.IRadialControllerControlAcquiredEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerControlAcquiredEventArgs2[] = L"Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_get_IsButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerControlAcquiredEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenu
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenu[] = L"Windows.UI.Input.IRadialControllerMenu";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        __FIVector_1_Windows__CUI__CInput__CRadialControllerMenuItem** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetSelectedMenuItem)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* SelectMenuItem)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* menuItem);
    HRESULT (STDMETHODCALLTYPE* TrySelectPreviouslySelectedMenuItem)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_GetSelectedMenuItem(This, result) \
    ((This)->lpVtbl->GetSelectedMenuItem(This, result))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_SelectMenuItem(This, menuItem) \
    ((This)->lpVtbl->SelectMenuItem(This, menuItem))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_TrySelectPreviouslySelectedMenuItem(This, result) \
    ((This)->lpVtbl->TrySelectPreviouslySelectedMenuItem(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenu_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItem[] = L"Windows.UI.Input.IRadialControllerMenuItem";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayText)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* add_Invoked)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CRadialControllerMenuItem_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Invoked)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_get_DisplayText(This, value) \
    ((This)->lpVtbl->get_DisplayText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_add_Invoked(This, handler, token) \
    ((This)->lpVtbl->add_Invoked(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_remove_Invoked(This, token) \
    ((This)->lpVtbl->remove_Invoked(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItemStatics[] = L"Windows.UI.Input.IRadialControllerMenuItemStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromIcon)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        HSTRING displayText,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* icon,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromKnownIcon)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics* This,
        HSTRING displayText,
        enum __x_ABI_CWindows_CUI_CInput_CRadialControllerMenuKnownIcon value,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_CreateFromIcon(This, displayText, icon, result) \
    ((This)->lpVtbl->CreateFromIcon(This, displayText, icon, result))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_CreateFromKnownIcon(This, displayText, value, result) \
    ((This)->lpVtbl->CreateFromKnownIcon(This, displayText, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerMenuItemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerMenuItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerMenuItemStatics2[] = L"Windows.UI.Input.IRadialControllerMenuItemStatics2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromFontGlyph)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        HSTRING displayText,
        HSTRING glyph,
        HSTRING fontFamily,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromFontGlyphWithUri)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2* This,
        HSTRING displayText,
        HSTRING glyph,
        HSTRING fontFamily,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* fontUri,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_CreateFromFontGlyph(This, displayText, glyph, fontFamily, result) \
    ((This)->lpVtbl->CreateFromFontGlyph(This, displayText, glyph, fontFamily, result))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_CreateFromFontGlyphWithUri(This, displayText, glyph, fontFamily, fontUri, result) \
    ((This)->lpVtbl->CreateFromFontGlyphWithUri(This, displayText, glyph, fontFamily, fontUri, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerMenuItemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerRotationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerRotationChangedEventArgs[] = L"Windows.UI.Input.IRadialControllerRotationChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RotationDeltaInDegrees)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_get_RotationDeltaInDegrees(This, value) \
    ((This)->lpVtbl->get_RotationDeltaInDegrees(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerRotationChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerRotationChangedEventArgs2[] = L"Windows.UI.Input.IRadialControllerRotationChangedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_get_IsButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerRotationChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContact
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContact
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContact[] = L"Windows.UI.Input.IRadialControllerScreenContact";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bounds)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_get_Bounds(This, value) \
    ((This)->lpVtbl->get_Bounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactContinuedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactContinuedEventArgs2[] = L"Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_get_IsButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactContinuedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactEndedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactEndedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_get_IsButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactEndedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactStartedEventArgs[] = L"Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerScreenContactStartedEventArgs2[] = L"Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsButtonPressed)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_get_IsButtonPressed(This, value) \
    ((This)->lpVtbl->get_IsButtonPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerScreenContactStartedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.IRadialControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RadialController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRadialControllerStatics[] = L"Windows.UI.Input.IRadialControllerStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* CreateForCurrentView)(__x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRadialControllerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRadialControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_CreateForCurrentView(This, result) \
    ((This)->lpVtbl->CreateForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRadialControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.IRightTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RightTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRightTappedEventArgs[] = L"Windows.UI.Input.IRightTappedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.IRightTappedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.RightTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_IRightTappedEventArgs2[] = L"Windows.UI.Input.IRightTappedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CIRightTappedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemButtonEventController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemButtonEventController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemButtonEventController[] = L"Windows.UI.Input.ISystemButtonEventController";
typedef struct __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SystemFunctionButtonPressed)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemFunctionButtonPressed)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SystemFunctionButtonReleased)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionButtonEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemFunctionButtonReleased)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SystemFunctionLockChanged)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemFunctionLockChanged)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SystemFunctionLockIndicatorChanged)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSystemButtonEventController_Windows__CUI__CInput__CSystemFunctionLockIndicatorChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemFunctionLockIndicatorChanged)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventController* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_add_SystemFunctionButtonPressed(This, handler, token) \
    ((This)->lpVtbl->add_SystemFunctionButtonPressed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_remove_SystemFunctionButtonPressed(This, token) \
    ((This)->lpVtbl->remove_SystemFunctionButtonPressed(This, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_add_SystemFunctionButtonReleased(This, handler, token) \
    ((This)->lpVtbl->add_SystemFunctionButtonReleased(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_remove_SystemFunctionButtonReleased(This, token) \
    ((This)->lpVtbl->remove_SystemFunctionButtonReleased(This, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_add_SystemFunctionLockChanged(This, handler, token) \
    ((This)->lpVtbl->add_SystemFunctionLockChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_remove_SystemFunctionLockChanged(This, token) \
    ((This)->lpVtbl->remove_SystemFunctionLockChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_add_SystemFunctionLockIndicatorChanged(This, handler, token) \
    ((This)->lpVtbl->add_SystemFunctionLockIndicatorChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_remove_SystemFunctionLockIndicatorChanged(This, token) \
    ((This)->lpVtbl->remove_SystemFunctionLockIndicatorChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemButtonEventController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemButtonEventControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemButtonEventController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemButtonEventControllerStatics[] = L"Windows.UI.Input.ISystemButtonEventControllerStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForDispatcherQueue)(__x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue* queue,
        __x_ABI_CWindows_CUI_CInput_CISystemButtonEventController** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_CreateForDispatcherQueue(This, queue, result) \
    ((This)->lpVtbl->CreateForDispatcherQueue(This, queue, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemButtonEventControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionButtonEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionButtonEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionButtonEventArgs[] = L"Windows.UI.Input.ISystemFunctionButtonEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionButtonEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionLockChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionLockChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionLockChangedEventArgs[] = L"Windows.UI.Input.ISystemFunctionLockChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLocked)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_get_IsLocked(This, value) \
    ((This)->lpVtbl->get_IsLocked(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ISystemFunctionLockIndicatorChangedEventArgs[] = L"Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IsIndicatorOn)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_get_IsIndicatorOn(This, value) \
    ((This)->lpVtbl->get_IsIndicatorOn(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CISystemFunctionLockIndicatorChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ITappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITappedEventArgs[] = L"Windows.UI.Input.ITappedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CITappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_TapCount)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CITappedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CITappedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CITappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs_get_TapCount(This, value) \
    ((This)->lpVtbl->get_TapCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.ITappedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITappedEventArgs2[] = L"Windows.UI.Input.ITappedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactCount)(__x_ABI_CWindows_CUI_CInput_CITappedEventArgs2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_get_ContactCount(This, value) \
    ((This)->lpVtbl->get_ContactCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITappedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITappedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.ITouchpadGesturesController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGesturesController
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGesturesController[] = L"Windows.UI.Input.ITouchpadGesturesController";
typedef struct __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedGestures)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalGestureKinds* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportedGestures)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalGestureKinds value);
    HRESULT (STDMETHODCALLTYPE* add_PointerPressed)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PointerPressed)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PointerMoved)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PointerMoved)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PointerReleased)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PointerReleased)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GlobalActionPerformed)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CTouchpadGesturesController_Windows__CUI__CInput__CTouchpadGlobalActionEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GlobalActionPerformed)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_get_SupportedGestures(This, value) \
    ((This)->lpVtbl->get_SupportedGestures(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_put_SupportedGestures(This, value) \
    ((This)->lpVtbl->put_SupportedGestures(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_add_PointerPressed(This, handler, token) \
    ((This)->lpVtbl->add_PointerPressed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_remove_PointerPressed(This, token) \
    ((This)->lpVtbl->remove_PointerPressed(This, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_add_PointerMoved(This, handler, token) \
    ((This)->lpVtbl->add_PointerMoved(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_remove_PointerMoved(This, token) \
    ((This)->lpVtbl->remove_PointerMoved(This, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_add_PointerReleased(This, handler, token) \
    ((This)->lpVtbl->add_PointerReleased(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_remove_PointerReleased(This, token) \
    ((This)->lpVtbl->remove_PointerReleased(This, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_add_GlobalActionPerformed(This, handler, token) \
    ((This)->lpVtbl->add_GlobalActionPerformed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_remove_GlobalActionPerformed(This, token) \
    ((This)->lpVtbl->remove_GlobalActionPerformed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.ITouchpadGesturesControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGesturesController
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGesturesControllerStatics[] = L"Windows.UI.Input.ITouchpadGesturesControllerStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* CreateForProcess)(__x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics* This,
        __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesController** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_CreateForProcess(This, result) \
    ((This)->lpVtbl->CreateForProcess(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGesturesControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Input.ITouchpadGlobalActionEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Input.TouchpadGlobalActionEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_ITouchpadGlobalActionEventArgs[] = L"Windows.UI.Input.ITouchpadGlobalActionEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CTouchpadGlobalAction* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerDevice)(__x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_get_PointerDevice(This, value) \
    ((This)->lpVtbl->get_PointerDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CITouchpadGlobalActionEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.AttachableInputObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IAttachableInputObject ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_AttachableInputObject_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_AttachableInputObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_AttachableInputObject[] = L"Windows.UI.Input.AttachableInputObject";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.CrossSlidingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ICrossSlidingEventArgs ** Default Interface **
 *    Windows.UI.Input.ICrossSlidingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_CrossSlidingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_CrossSlidingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_CrossSlidingEventArgs[] = L"Windows.UI.Input.CrossSlidingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.DraggingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IDraggingEventArgs ** Default Interface **
 *    Windows.UI.Input.IDraggingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_DraggingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_DraggingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_DraggingEventArgs[] = L"Windows.UI.Input.DraggingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.EdgeGesture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IEdgeGestureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IEdgeGesture ** Default Interface **
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_EdgeGesture_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_EdgeGesture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_EdgeGesture[] = L"Windows.UI.Input.EdgeGesture";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.EdgeGestureEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IEdgeGestureEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_EdgeGestureEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_EdgeGestureEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_EdgeGestureEventArgs[] = L"Windows.UI.Input.EdgeGestureEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.GestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IGestureRecognizer ** Default Interface **
 *    Windows.UI.Input.IGestureRecognizer2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_GestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_GestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_GestureRecognizer[] = L"Windows.UI.Input.GestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.HoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IHoldingEventArgs ** Default Interface **
 *    Windows.UI.Input.IHoldingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_HoldingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_HoldingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_HoldingEventArgs[] = L"Windows.UI.Input.HoldingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.InputActivationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IInputActivationListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_InputActivationListener_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_InputActivationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_InputActivationListener[] = L"Windows.UI.Input.InputActivationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.InputActivationListenerActivationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_InputActivationListenerActivationChangedEventArgs[] = L"Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Input.KeyboardDeliveryInterceptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IKeyboardDeliveryInterceptorStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IKeyboardDeliveryInterceptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_KeyboardDeliveryInterceptor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_KeyboardDeliveryInterceptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_KeyboardDeliveryInterceptor[] = L"Windows.UI.Input.KeyboardDeliveryInterceptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.ManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationCompletedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationCompletedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationCompletedEventArgs[] = L"Windows.UI.Input.ManipulationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationInertiaStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationInertiaStartingEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationInertiaStartingEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationInertiaStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationInertiaStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationInertiaStartingEventArgs[] = L"Windows.UI.Input.ManipulationInertiaStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationStartedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationStartedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationStartedEventArgs[] = L"Windows.UI.Input.ManipulationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.ManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IManipulationUpdatedEventArgs ** Default Interface **
 *    Windows.UI.Input.IManipulationUpdatedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_ManipulationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_ManipulationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_ManipulationUpdatedEventArgs[] = L"Windows.UI.Input.ManipulationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.MouseWheelParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IMouseWheelParameters ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_MouseWheelParameters_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_MouseWheelParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_MouseWheelParameters[] = L"Windows.UI.Input.MouseWheelParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PhysicalGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPhysicalGestureRecognizer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_PhysicalGestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PhysicalGestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PhysicalGestureRecognizer[] = L"Windows.UI.Input.PhysicalGestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.PointerPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IPointerPointStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerPoint ** Default Interface **
 *    Windows.UI.Input.IPointerPointPhysicalPosition
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerPoint_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerPoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerPoint[] = L"Windows.UI.Input.PointerPoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PointerPointProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerPointProperties ** Default Interface **
 *    Windows.UI.Input.IPointerPointProperties2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerPointProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerPointProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerPointProperties[] = L"Windows.UI.Input.PointerPointProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.PointerVisualizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IPointerVisualizationSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IPointerVisualizationSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_PointerVisualizationSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_PointerVisualizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_PointerVisualizationSettings[] = L"Windows.UI.Input.PointerVisualizationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.RadialController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialController ** Default Interface **
 *    Windows.UI.Input.IRadialController2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialController[] = L"Windows.UI.Input.RadialController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonClickedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerButtonClickedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonClickedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonHoldingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonHoldingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonHoldingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonHoldingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonHoldingEventArgs[] = L"Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonPressedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerButtonReleasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerButtonReleasedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonReleasedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerButtonReleasedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerButtonReleasedEventArgs[] = L"Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerConfigurationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerConfigurationStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerConfiguration ** Default Interface **
 *    Windows.UI.Input.IRadialControllerConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerConfiguration[] = L"Windows.UI.Input.RadialControllerConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerControlAcquiredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerControlAcquiredEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerControlAcquiredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerControlAcquiredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerControlAcquiredEventArgs[] = L"Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerMenu ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerMenu_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerMenu_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerMenu[] = L"Windows.UI.Input.RadialControllerMenu";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerMenuItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerMenuItemStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.IRadialControllerMenuItemStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerMenuItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerMenuItem_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerMenuItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerMenuItem[] = L"Windows.UI.Input.RadialControllerMenuItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerRotationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerRotationChangedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerRotationChangedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerRotationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerRotationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerRotationChangedEventArgs[] = L"Windows.UI.Input.RadialControllerRotationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContact
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContact ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContact_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContact_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContact[] = L"Windows.UI.Input.RadialControllerScreenContact";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactContinuedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactEndedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.RadialControllerScreenContactStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RadialControllerScreenContactStartedEventArgs[] = L"Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.RightTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.IRightTappedEventArgs ** Default Interface **
 *    Windows.UI.Input.IRightTappedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_RightTappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_RightTappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_RightTappedEventArgs[] = L"Windows.UI.Input.RightTappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.SystemButtonEventController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.ISystemButtonEventControllerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemButtonEventController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemButtonEventController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemButtonEventController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemButtonEventController[] = L"Windows.UI.Input.SystemButtonEventController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionButtonEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionButtonEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionButtonEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionButtonEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionButtonEventArgs[] = L"Windows.UI.Input.SystemFunctionButtonEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionLockChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionLockChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionLockChangedEventArgs[] = L"Windows.UI.Input.SystemFunctionLockChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_SystemFunctionLockIndicatorChangedEventArgs[] = L"Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.Input.TappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITappedEventArgs ** Default Interface **
 *    Windows.UI.Input.ITappedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_TappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TappedEventArgs[] = L"Windows.UI.Input.TappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.TouchpadGesturesController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.ITouchpadGesturesControllerStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITouchpadGesturesController ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_TouchpadGesturesController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TouchpadGesturesController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TouchpadGesturesController[] = L"Windows.UI.Input.TouchpadGesturesController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Input.TouchpadGlobalActionEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.ITouchpadGlobalActionEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_UI_Input_TouchpadGlobalActionEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_TouchpadGlobalActionEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_TouchpadGlobalActionEventArgs[] = L"Windows.UI.Input.TouchpadGlobalActionEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput_p_h__

#endif // __windows2Eui2Einput_h__
