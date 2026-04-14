
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
#ifndef __windows2Eui2Einput2Espatial_h__
#define __windows2Eui2Einput2Espatial_h__
#ifndef __windows2Eui2Einput2Espatial_p_h__
#define __windows2Eui2Einput2Espatial_p_h__


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
#if !defined(WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION)
#define WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION)

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
#include "Windows.Devices.Haptics.h"
#include "Windows.Devices.Power.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Perception.h"
#include "Windows.Perception.People.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialGestureRecognizer;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialGestureRecognizerFactory;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizerFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialHoldCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs ABI::Windows::UI::Input::Spatial::ISpatialHoldCanceledEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialHoldCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialHoldCompletedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialHoldStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialHoldStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteraction;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction ABI::Windows::UI::Input::Spatial::ISpatialInteraction

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionController;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController ABI::Windows::UI::Input::Spatial::ISpatialInteractionController

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionController2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionController2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionController3;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3 ABI::Windows::UI::Input::Spatial::ISpatialInteractionController3

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionControllerProperties;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties ABI::Windows::UI::Input::Spatial::ISpatialInteractionControllerProperties

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionDetectedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialInteractionDetectedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionDetectedEventArgs2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionDetectedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionManager;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager ABI::Windows::UI::Input::Spatial::ISpatialInteractionManager

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionManagerStatics;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics ABI::Windows::UI::Input::Spatial::ISpatialInteractionManagerStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionManagerStatics2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionManagerStatics2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSource;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSource2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSource3;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource3

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSource4;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource4

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceEventArgs2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceEventArgs2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceLocation;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceLocation

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceLocation2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceLocation2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceLocation3;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceLocation3

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceProperties;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceProperties

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceState;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceState2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSourceState3;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3 ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState3

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialManipulationCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs ABI::Windows::UI::Input::Spatial::ISpatialManipulationCanceledEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialManipulationCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialManipulationCompletedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialManipulationDelta;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta ABI::Windows::UI::Input::Spatial::ISpatialManipulationDelta

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialManipulationStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialManipulationStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialManipulationUpdatedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialManipulationUpdatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialNavigationCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs ABI::Windows::UI::Input::Spatial::ISpatialNavigationCanceledEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialNavigationCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialNavigationCompletedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialNavigationStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialNavigationStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialNavigationUpdatedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialNavigationUpdatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerInteractionSourcePose;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose ABI::Windows::UI::Input::Spatial::ISpatialPointerInteractionSourcePose

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerInteractionSourcePose2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2 ABI::Windows::UI::Input::Spatial::ISpatialPointerInteractionSourcePose2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerPose;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose ABI::Windows::UI::Input::Spatial::ISpatialPointerPose

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerPose2;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2 ABI::Windows::UI::Input::Spatial::ISpatialPointerPose2

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerPose3;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3 ABI::Windows::UI::Input::Spatial::ISpatialPointerPose3

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialPointerPoseStatics;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics ABI::Windows::UI::Input::Spatial::ISpatialPointerPoseStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialRecognitionEndedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialRecognitionEndedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialRecognitionStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialRecognitionStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialTappedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs ABI::Windows::UI::Input::Spatial::ISpatialTappedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                class HandMeshObserver;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHandMeshObserver;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver ABI::Windows::Perception::People::IHandMeshObserver

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0bfeea40-9b97-5abf-af9e-2e6367a50bf8"))
IAsyncOperation<ABI::Windows::Perception::People::HandMeshObserver*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::People::HandMeshObserver*, ABI::Windows::Perception::People::IHandMeshObserver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.People.HandMeshObserver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Perception::People::HandMeshObserver*> __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_t;
#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("75e7a8a7-b66d-5e6b-a060-eef7002d9e62"))
IAsyncOperationCompletedHandler<ABI::Windows::Perception::People::HandMeshObserver*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::People::HandMeshObserver*, ABI::Windows::Perception::People::IHandMeshObserver*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.People.HandMeshObserver>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Perception::People::HandMeshObserver*> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4a57c5e-32b0-55b3-ad13-ce1c23041ed6"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3dddecf4-1d39-58e8-83b1-dbed541c7f35"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSourceState;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23f50032-907e-5d73-bbdc-f7563de655c4"))
IIterator<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Spatial.SpatialInteractionSourceState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t;
#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7ecce0f5-0874-506f-8d42-4c9519156407"))
IIterable<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Spatial.SpatialInteractionSourceState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t;
#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dfdcf091-6e3c-5e5b-a193-03acc414c921"))
IVectorView<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Spatial.SpatialInteractionSourceState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceState*> __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t;
#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_USE
#define DEF___FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b27004bb-c014-5dce-9a21-799c5a3c1461"))
IReference<struct ABI::Windows::Foundation::Numerics::Quaternion> : IReference_impl<struct ABI::Windows::Foundation::Numerics::Quaternion>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Numerics.Quaternion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Numerics::Quaternion> __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_t;
#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CNumerics__CVector3_USE
#define DEF___FIReference_1_Windows__CFoundation__CNumerics__CVector3_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1ee770ff-c954-59ca-a754-6199a9be282c"))
IReference<struct ABI::Windows::Foundation::Numerics::Vector3> : IReference_impl<struct ABI::Windows::Foundation::Numerics::Vector3>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Numerics.Vector3>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Numerics::Vector3> __FIReference_1_Windows__CFoundation__CNumerics__CVector3_t;
#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3 ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CNumerics__CVector3_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CNumerics__CVector3_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialGestureRecognizer;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialHoldCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6a715963-c49f-5f3d-bfec-952700308860"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldCanceledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialHoldCanceledEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialHoldCanceledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldCanceledEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialHoldCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e425d80e-8c18-5375-b600-1dd7a11dba5e"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialHoldCompletedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialHoldCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialHoldStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7537dd12-02b8-5132-8ff7-90c80ef454d1"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialHoldStartedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialHoldStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialHoldStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialManipulationCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b92a6754-3946-5353-b8f9-2a07eb0e8886"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationCanceledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialManipulationCanceledEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialManipulationCanceledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationCanceledEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialManipulationCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10dac914-bea2-5d67-8607-dc470ea1dcd8"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialManipulationCompletedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialManipulationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialManipulationStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a123252b-cfe8-5cfb-b61c-f2dc011eb12e"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialManipulationStartedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialManipulationStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialManipulationUpdatedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cecbe023-3c49-530e-a010-8c0c3cbd3088"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialManipulationUpdatedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialManipulationUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialManipulationUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialNavigationCanceledEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9947ea8e-eb4d-5f93-9fd9-2ade6470bc5d"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationCanceledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialNavigationCanceledEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialNavigationCanceledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationCanceledEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialNavigationCompletedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f0d459df-ebd3-5388-b0a9-5f44bcd6f58f"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialNavigationCompletedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialNavigationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialNavigationStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5419cadc-2600-527b-9398-1744318d0618"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialNavigationStartedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialNavigationStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialNavigationUpdatedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a7216b90-c175-5a28-b500-a445f0751c5b"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationUpdatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialNavigationUpdatedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialNavigationUpdatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialNavigationUpdatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialRecognitionEndedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("afa7fad7-3fd7-5c81-8802-195c523f6e7b"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialRecognitionEndedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialRecognitionEndedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialRecognitionEndedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialRecognitionEndedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialRecognitionStartedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f2bd99d6-99fa-5599-a14a-1f7a7a92e3d7"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialRecognitionStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialRecognitionStartedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialRecognitionStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialRecognitionStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialTappedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("630205a4-2f93-5022-b7b8-c43ed428498a"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialTappedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialTappedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialTappedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialGestureRecognizer, Windows.UI.Input.Spatial.SpatialTappedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialGestureRecognizer*, ABI::Windows::UI::Input::Spatial::SpatialTappedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionManager;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionDetectedEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0f223f8f-f220-5388-8fe6-b3b7e7930987"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::SpatialInteractionDetectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionDetectedEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionDetectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialInteractionManager, Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::SpatialInteractionDetectedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSourceEventArgs;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ae831d66-70f0-5371-866f-a3fc8b60e2d8"))
ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceEventArgs*, ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Spatial.SpatialInteractionManager, Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Spatial::SpatialInteractionManager*, ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

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
            namespace Power {
                class BatteryReport;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IBatteryReport;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport ABI::Windows::Devices::Power::IBatteryReport

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__

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
        namespace Perception {
            namespace People {
                class EyesPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IEyesPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose ABI::Windows::Perception::People::IEyesPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                class HandPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHandPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose ABI::Windows::Perception::People::IHandPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                class HeadPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHeadPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose ABI::Windows::Perception::People::IHeadPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            class PerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            interface IPerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CIPerceptionTimestamp ABI::Windows::Perception::IPerceptionTimestamp

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    typedef enum SpatialGestureSettings : unsigned int SpatialGestureSettings;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    typedef enum SpatialInteractionPressKind : int SpatialInteractionPressKind;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    typedef enum SpatialInteractionSourceHandedness : int SpatialInteractionSourceHandedness;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    typedef enum SpatialInteractionSourceKind : int SpatialInteractionSourceKind;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    typedef enum SpatialInteractionSourcePositionAccuracy : int SpatialInteractionSourcePositionAccuracy;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteraction;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionController;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionControllerProperties;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSource;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSourceLocation;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSourceProperties;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialManipulationDelta;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialPointerInteractionSourcePose;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialPointerPose;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialGestureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    enum SpatialGestureSettings : unsigned int
                    {
                        SpatialGestureSettings_None = 0,
                        SpatialGestureSettings_Tap = 0x1,
                        SpatialGestureSettings_DoubleTap = 0x2,
                        SpatialGestureSettings_Hold = 0x4,
                        SpatialGestureSettings_ManipulationTranslate = 0x8,
                        SpatialGestureSettings_NavigationX = 0x10,
                        SpatialGestureSettings_NavigationY = 0x20,
                        SpatialGestureSettings_NavigationZ = 0x40,
                        SpatialGestureSettings_NavigationRailsX = 0x80,
                        SpatialGestureSettings_NavigationRailsY = 0x100,
                        SpatialGestureSettings_NavigationRailsZ = 0x200,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(SpatialGestureSettings)
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionPressKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    enum SpatialInteractionPressKind : int
                    {
                        SpatialInteractionPressKind_None = 0,
                        SpatialInteractionPressKind_Select = 1,
                        SpatialInteractionPressKind_Menu = 2,
                        SpatialInteractionPressKind_Grasp = 3,
                        SpatialInteractionPressKind_Touchpad = 4,
                        SpatialInteractionPressKind_Thumbstick = 5,
                    };
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourceHandedness
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    enum SpatialInteractionSourceHandedness : int
                    {
                        SpatialInteractionSourceHandedness_Unspecified = 0,
                        SpatialInteractionSourceHandedness_Left = 1,
                        SpatialInteractionSourceHandedness_Right = 2,
                    };
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    enum SpatialInteractionSourceKind : int
                    {
                        SpatialInteractionSourceKind_Other = 0,
                        SpatialInteractionSourceKind_Hand = 1,
                        SpatialInteractionSourceKind_Voice = 2,
                        SpatialInteractionSourceKind_Controller = 3,
                    };
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourcePositionAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    enum SpatialInteractionSourcePositionAccuracy : int
                    {
                        SpatialInteractionSourcePositionAccuracy_High = 0,
                        SpatialInteractionSourcePositionAccuracy_Approximate = 1,
                    };
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialGestureRecognizer[] = L"Windows.UI.Input.Spatial.ISpatialGestureRecognizer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("71605bcc-0c35-4673-adbd-cc04caa6ef45")
                    ISpatialGestureRecognizer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_RecognitionStarted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RecognitionStarted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RecognitionEnded(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RecognitionEnded(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Tapped(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Tapped(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_HoldStarted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_HoldStarted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_HoldCompleted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_HoldCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_HoldCanceled(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_HoldCanceled(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ManipulationStarted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ManipulationStarted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ManipulationUpdated(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ManipulationUpdated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ManipulationCompleted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ManipulationCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ManipulationCanceled(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ManipulationCanceled(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NavigationStarted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NavigationStarted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NavigationUpdated(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NavigationUpdated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NavigationCompleted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NavigationCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NavigationCanceled(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NavigationCanceled(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CaptureInteraction(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteraction* interaction
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CancelPendingGestures(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TrySetGestureSettings(
                            ABI::Windows::UI::Input::Spatial::SpatialGestureSettings settings,
                            boolean* succeeded
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_GestureSettings(
                            ABI::Windows::UI::Input::Spatial::SpatialGestureSettings* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialGestureRecognizer = __uuidof(ISpatialGestureRecognizer);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialGestureRecognizerFactory[] = L"Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("77214186-57b9-3150-8382-698b24e264d0")
                    ISpatialGestureRecognizerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Input::Spatial::SpatialGestureSettings settings,
                            ABI::Windows::UI::Input::Spatial::ISpatialGestureRecognizer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialGestureRecognizerFactory = __uuidof(ISpatialGestureRecognizerFactory);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("5dfcb667-4caa-4093-8c35-b601a839f31b")
                    ISpatialHoldCanceledEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialHoldCanceledEventArgs = __uuidof(ISpatialHoldCanceledEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("3f64470b-4cfd-43da-8dc4-e64552173971")
                    ISpatialHoldCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialHoldCompletedEventArgs = __uuidof(ISpatialHoldCompletedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("8e343d79-acb6-4144-8615-2cfba8a3cb3f")
                    ISpatialHoldStartedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialHoldStartedEventArgs = __uuidof(ISpatialHoldStartedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteraction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteraction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteraction[] = L"Windows.UI.Input.Spatial.ISpatialInteraction";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("fc967639-88e6-4646-9112-4344aaec9dfa")
                    ISpatialInteraction : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceState(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteraction = __uuidof(ISpatialInteraction);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("5f0e5ba3-0954-4e97-86c5-e7f30b114dfd")
                    ISpatialInteractionController : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HasTouchpad(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasThumbstick(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                            ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VendorId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Version(
                            UINT16* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionController = __uuidof(ISpatialInteractionController);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("35b6d924-c7a2-49b7-b72e-5436b2fb8f9c")
                    ISpatialInteractionController2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetRenderableModelAsync(
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionController2 = __uuidof(ISpatialInteractionController2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionController2
 *     Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("628466a0-9d91-4a0b-888d-165e670a8cd5")
                    ISpatialInteractionController3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetBatteryReport(
                            ABI::Windows::Devices::Power::IBatteryReport** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionController3 = __uuidof(ISpatialInteractionController3);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionControllerProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionControllerProperties[] = L"Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("61056fb1-7ba9-4e35-b93f-9272cba9b28b")
                    ISpatialInteractionControllerProperties : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTouchpadTouched(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTouchpadPressed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsThumbstickPressed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThumbstickX(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThumbstickY(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TouchpadX(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TouchpadY(
                            DOUBLE* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionControllerProperties = __uuidof(ISpatialInteractionControllerProperties);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionDetectedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("075878e4-5961-3b41-9dfb-cea5d89cc38a")
                    ISpatialInteractionDetectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Interaction(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteraction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionDetectedEventArgs = __uuidof(ISpatialInteractionDetectedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionDetectedEventArgs2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("7b263e93-5f13-419c-97d5-834678266aa6")
                    ISpatialInteractionDetectedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSource(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionDetectedEventArgs2 = __uuidof(ISpatialInteractionDetectedEventArgs2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManager[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("32a64ea8-a15a-3995-b8bd-80513cb5adef")
                    ISpatialInteractionManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_SourceDetected(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SourceDetected(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SourceLost(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SourceLost(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SourceUpdated(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SourceUpdated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SourcePressed(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SourcePressed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SourceReleased(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SourceReleased(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_InteractionDetected(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_InteractionDetected(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDetectedSourcesAtTimestamp(
                            ABI::Windows::Perception::IPerceptionTimestamp* timeStamp,
                            __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionManager = __uuidof(ISpatialInteractionManager);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManagerStatics[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("00e31fa6-8ca2-30bf-91fe-d9cb4a008990")
                    ISpatialInteractionManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionManager** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionManagerStatics = __uuidof(ISpatialInteractionManagerStatics);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManagerStatics2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("93f16c52-b88a-5929-8d7c-48cb948b081c")
                    ISpatialInteractionManagerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsSourceKindSupported(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind kind,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionManagerStatics2 = __uuidof(ISpatialInteractionManagerStatics2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("fb5433ba-b0b3-3148-9f3b-e9f5de568f5d")
                    ISpatialInteractionSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSource = __uuidof(ISpatialInteractionSource);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("e4c5b70c-0470-4028-88c0-a0eb44d34efe")
                    ISpatialInteractionSource2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPointingSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMenuSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsGraspSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Controller(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionController** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetStateAtTimestamp(
                            ABI::Windows::Perception::IPerceptionTimestamp* timestamp,
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSource2 = __uuidof(ISpatialInteractionSource2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("0406d9f9-9afd-44f9-85dc-700023a962e3")
                    ISpatialInteractionSource3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handedness(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceHandedness* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSource3 = __uuidof(ISpatialInteractionSource3);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource4[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("0073bc4d-df66-5a91-a2ba-cea3e5c58a19")
                    ISpatialInteractionSource4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryCreateHandMeshObserver(
                            ABI::Windows::Perception::People::IHandMeshObserver** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryCreateHandMeshObserverAsync(
                            __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSource4 = __uuidof(ISpatialInteractionSource4);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("23b786cf-ec23-3979-b27c-eb0e12feb7c7")
                    ISpatialInteractionSourceEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceState** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceEventArgs = __uuidof(ISpatialInteractionSourceEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceEventArgs2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("d8b4b467-e648-4d52-ab49-e0d227199f63")
                    ISpatialInteractionSourceEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PressKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionPressKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceEventArgs2 = __uuidof(ISpatialInteractionSourceEventArgs2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("ea4696c4-7e8b-30ca-bcc5-c77189cea30a")
                    ISpatialInteractionSourceLocation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Velocity(
                            __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceLocation = __uuidof(ISpatialInteractionSourceLocation);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("4c671045-3917-40fc-a9ac-31c9cf5ff91b")
                    ISpatialInteractionSourceLocation2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                            __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceLocation2 = __uuidof(ISpatialInteractionSourceLocation2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("6702e65e-e915-4cfb-9c1b-0538efc86687")
                    ISpatialInteractionSourceLocation3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PositionAccuracy(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourcePositionAccuracy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AngularVelocity(
                            __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePointerPose(
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerInteractionSourcePose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceLocation3 = __uuidof(ISpatialInteractionSourceLocation3);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceProperties[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("05604542-3ef7-3222-9f53-63c9cb7e3bc7")
                    ISpatialInteractionSourceProperties : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetSourceLossMitigationDirection(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceLossRisk(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetLocation(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceLocation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceProperties = __uuidof(ISpatialInteractionSourceProperties);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("d5c475ef-4b63-37ec-98b9-9fc652b9d2f2")
                    ISpatialInteractionSourceState : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Source(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSourceProperties** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsPressed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Perception::IPerceptionTimestamp** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceState = __uuidof(ISpatialInteractionSourceState);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("45f6d0bd-1773-492e-9ba3-8ac1cbe77c08")
                    ISpatialInteractionSourceState2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsSelectPressed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMenuPressed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsGrasped(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectPressedValue(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ControllerProperties(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionControllerProperties** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceState2 = __uuidof(ISpatialInteractionSourceState2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("f2f00bc2-bd2b-4a01-a8fb-323e0158527c")
                    ISpatialInteractionSourceState3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetHandPose(
                            ABI::Windows::Perception::People::IHandPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialInteractionSourceState3 = __uuidof(ISpatialInteractionSourceState3);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("2d40d1cb-e7da-4220-b0bf-819301674780")
                    ISpatialManipulationCanceledEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialManipulationCanceledEventArgs = __uuidof(ISpatialManipulationCanceledEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("05086802-f301-4343-9250-2fbaa5f87a37")
                    ISpatialManipulationCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetCumulativeDelta(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialManipulationDelta** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialManipulationCompletedEventArgs = __uuidof(ISpatialManipulationCompletedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationDelta
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationDelta[] = L"Windows.UI.Input.Spatial.ISpatialManipulationDelta";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("a7ec967a-d123-3a81-a15b-992923dcbe91")
                    ISpatialManipulationDelta : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Translation(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialManipulationDelta = __uuidof(ISpatialManipulationDelta);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("a1d6bbce-42a5-377b-ada6-d28e3d384737")
                    ISpatialManipulationStartedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialManipulationStartedEventArgs = __uuidof(ISpatialManipulationStartedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("5f230b9b-60c6-4dc6-bdc9-9f4a6f15fe49")
                    ISpatialManipulationUpdatedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetCumulativeDelta(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialManipulationDelta** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialManipulationUpdatedEventArgs = __uuidof(ISpatialManipulationUpdatedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("ce503edc-e8a5-46f0-92d4-3c122b35112a")
                    ISpatialNavigationCanceledEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialNavigationCanceledEventArgs = __uuidof(ISpatialNavigationCanceledEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("012e80b7-af3b-42c2-9e41-baaa0e721f3a")
                    ISpatialNavigationCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NormalizedOffset(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialNavigationCompletedEventArgs = __uuidof(ISpatialNavigationCompletedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("754a348a-fb64-4656-8ebd-9deecaafe475")
                    ISpatialNavigationStartedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsNavigatingX(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsNavigatingY(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsNavigatingZ(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialNavigationStartedEventArgs = __uuidof(ISpatialNavigationStartedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("9b713fd7-839d-4a74-8732-45466fc044b5")
                    ISpatialNavigationUpdatedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NormalizedOffset(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialNavigationUpdatedEventArgs = __uuidof(ISpatialNavigationUpdatedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerInteractionSourcePose[] = L"Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("a7104307-2c2b-4d3a-92a7-80ced7c4a0d0")
                    ISpatialPointerInteractionSourcePose : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForwardDirection(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UpDirection(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerInteractionSourcePose = __uuidof(ISpatialPointerInteractionSourcePose);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerInteractionSourcePose2[] = L"Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("eccd86b8-52db-469f-9e3f-80c47f74bce9")
                    ISpatialPointerInteractionSourcePose2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                            ABI::Windows::Foundation::Numerics::Quaternion* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PositionAccuracy(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourcePositionAccuracy* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerInteractionSourcePose2 = __uuidof(ISpatialPointerInteractionSourcePose2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("6953a42e-c17e-357d-97a1-7269d0ed2d10")
                    ISpatialPointerPose : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Perception::IPerceptionTimestamp** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Head(
                            ABI::Windows::Perception::People::IHeadPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerPose = __uuidof(ISpatialPointerPose);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose2[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("9d202b17-954e-4e0c-96d1-b6790b6fc2fd")
                    ISpatialPointerPose2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetInteractionSourcePose(
                            ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource* source,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerInteractionSourcePose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerPose2 = __uuidof(ISpatialPointerPose2);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose3[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("6342f3f0-ec49-5b4b-b8d1-d16cbb16be84")
                    ISpatialPointerPose3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Eyes(
                            ABI::Windows::Perception::People::IEyesPose** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsHeadCapturedBySystem(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerPose3 = __uuidof(ISpatialPointerPose3);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPoseStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPoseStatics[] = L"Windows.UI.Input.Spatial.ISpatialPointerPoseStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("a25591a9-aca1-3ee0-9816-785cfb2e3fb8")
                    ISpatialPointerPoseStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetAtTimestamp(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::Perception::IPerceptionTimestamp* timestamp,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialPointerPoseStatics = __uuidof(ISpatialPointerPoseStatics);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialRecognitionEndedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("0e35f5cb-3f75-43f3-ac81-d1dc2df9b1fb")
                    ISpatialRecognitionEndedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialRecognitionEndedEventArgs = __uuidof(ISpatialRecognitionEndedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialRecognitionStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("24da128f-0008-4a6d-aa50-2a76f9cfb264")
                    ISpatialRecognitionStartedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsGesturePossible(
                            ABI::Windows::UI::Input::Spatial::SpatialGestureSettings gesture,
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialRecognitionStartedEventArgs = __uuidof(ISpatialRecognitionStartedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialTappedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialTappedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    MIDL_INTERFACE("296d83de-f444-4aa1-b2bf-9dc88d567da6")
                    ISpatialTappedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InteractionSourceKind(
                            ABI::Windows::UI::Input::Spatial::SpatialInteractionSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetPointerPose(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::UI::Input::Spatial::ISpatialPointerPose** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TapCount(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialTappedEventArgs = __uuidof(ISpatialTappedEventArgs);
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialGestureRecognizer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialGestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialGestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialGestureRecognizer[] = L"Windows.UI.Input.Spatial.SpatialGestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteraction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteraction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteraction_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteraction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteraction[] = L"Windows.UI.Input.Spatial.SpatialInteraction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionController ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionController2
 *    Windows.UI.Input.Spatial.ISpatialInteractionController3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionController[] = L"Windows.UI.Input.Spatial.SpatialInteractionController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionControllerProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties[] = L"Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionManager[] = L"Windows.UI.Input.Spatial.SpatialInteractionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource3
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSource[] = L"Windows.UI.Input.Spatial.SpatialInteractionSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceState_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceState[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationDelta ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationDelta_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationDelta_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationDelta[] = L"Windows.UI.Input.Spatial.SpatialManipulationDelta";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose[] = L"Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialPointerPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialPointerPoseStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialPointerPose ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialPointerPose2
 *    Windows.UI.Input.Spatial.ISpatialPointerPose3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerPose_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialPointerPose[] = L"Windows.UI.Input.Spatial.SpatialPointerPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialTappedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialTappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialTappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialTappedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialTappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3 __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver* This,
        __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CPeople__CHandMeshObserver_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

typedef struct __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

typedef struct __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        __FIIterator_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CNumerics__CQuaternion;

typedef struct __FIReference_1_Windows__CFoundation__CNumerics__CQuaternionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CNumerics__CQuaternion* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CNumerics__CQuaternionVtbl;

interface __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CNumerics__CQuaternionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CNumerics__CQuaternion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CNumerics__CVector3_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CNumerics__CVector3_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CNumerics__CVector3 __FIReference_1_Windows__CFoundation__CNumerics__CVector3;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CNumerics__CVector3;

typedef struct __FIReference_1_Windows__CFoundation__CNumerics__CVector3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CNumerics__CVector3* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CNumerics__CVector3Vtbl;

interface __FIReference_1_Windows__CFoundation__CNumerics__CVector3
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CNumerics__CVector3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CNumerics__CVector3_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CNumerics__CVector3_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* sender,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIBatteryReport __x_ABI_CWindows_CDevices_CPower_CIBatteryReport;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIEyesPose __x_ABI_CWindows_CPerception_CPeople_CIEyesPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHandPose __x_ABI_CWindows_CPerception_CPeople_CIHandPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHeadPose __x_ABI_CWindows_CPerception_CPeople_CIHeadPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CIPerceptionTimestamp __x_ABI_CWindows_CPerception_CIPerceptionTimestamp;

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings;

typedef enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionPressKind __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionPressKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceHandedness __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceHandedness;

typedef enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourcePositionAccuracy __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourcePositionAccuracy;

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialGestureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings
{
    SpatialGestureSettings_None = 0,
    SpatialGestureSettings_Tap = 0x1,
    SpatialGestureSettings_DoubleTap = 0x2,
    SpatialGestureSettings_Hold = 0x4,
    SpatialGestureSettings_ManipulationTranslate = 0x8,
    SpatialGestureSettings_NavigationX = 0x10,
    SpatialGestureSettings_NavigationY = 0x20,
    SpatialGestureSettings_NavigationZ = 0x40,
    SpatialGestureSettings_NavigationRailsX = 0x80,
    SpatialGestureSettings_NavigationRailsY = 0x100,
    SpatialGestureSettings_NavigationRailsZ = 0x200,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionPressKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionPressKind
{
    SpatialInteractionPressKind_None = 0,
    SpatialInteractionPressKind_Select = 1,
    SpatialInteractionPressKind_Menu = 2,
    SpatialInteractionPressKind_Grasp = 3,
    SpatialInteractionPressKind_Touchpad = 4,
    SpatialInteractionPressKind_Thumbstick = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourceHandedness
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceHandedness
{
    SpatialInteractionSourceHandedness_Unspecified = 0,
    SpatialInteractionSourceHandedness_Left = 1,
    SpatialInteractionSourceHandedness_Right = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind
{
    SpatialInteractionSourceKind_Other = 0,
    SpatialInteractionSourceKind_Hand = 1,
    SpatialInteractionSourceKind_Voice = 2,
    SpatialInteractionSourceKind_Controller = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.Input.Spatial.SpatialInteractionSourcePositionAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourcePositionAccuracy
{
    SpatialInteractionSourcePositionAccuracy_High = 0,
    SpatialInteractionSourcePositionAccuracy_Approximate = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialGestureRecognizer[] = L"Windows.UI.Input.Spatial.ISpatialGestureRecognizer";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_RecognitionStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RecognitionStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RecognitionEnded)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialRecognitionEndedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RecognitionEnded)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Tapped)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialTappedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Tapped)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HoldStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HoldStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HoldCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HoldCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HoldCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialHoldCanceledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HoldCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ManipulationCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialManipulationCanceledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ManipulationCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NavigationStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationStarted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NavigationUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationUpdatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NavigationCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationCompleted)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NavigationCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialGestureRecognizer_Windows__CUI__CInput__CSpatial__CSpatialNavigationCanceledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationCanceled)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CaptureInteraction)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* interaction);
    HRESULT (STDMETHODCALLTYPE* CancelPendingGestures)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* TrySetGestureSettings)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings settings,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* get_GestureSettings)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_RecognitionStarted(This, handler, token) \
    ((This)->lpVtbl->add_RecognitionStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_RecognitionStarted(This, token) \
    ((This)->lpVtbl->remove_RecognitionStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_RecognitionEnded(This, handler, token) \
    ((This)->lpVtbl->add_RecognitionEnded(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_RecognitionEnded(This, token) \
    ((This)->lpVtbl->remove_RecognitionEnded(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_Tapped(This, handler, token) \
    ((This)->lpVtbl->add_Tapped(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_Tapped(This, token) \
    ((This)->lpVtbl->remove_Tapped(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_HoldStarted(This, handler, token) \
    ((This)->lpVtbl->add_HoldStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_HoldStarted(This, token) \
    ((This)->lpVtbl->remove_HoldStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_HoldCompleted(This, handler, token) \
    ((This)->lpVtbl->add_HoldCompleted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_HoldCompleted(This, token) \
    ((This)->lpVtbl->remove_HoldCompleted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_HoldCanceled(This, handler, token) \
    ((This)->lpVtbl->add_HoldCanceled(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_HoldCanceled(This, token) \
    ((This)->lpVtbl->remove_HoldCanceled(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_ManipulationStarted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_ManipulationStarted(This, token) \
    ((This)->lpVtbl->remove_ManipulationStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_ManipulationUpdated(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationUpdated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_ManipulationUpdated(This, token) \
    ((This)->lpVtbl->remove_ManipulationUpdated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_ManipulationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationCompleted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_ManipulationCompleted(This, token) \
    ((This)->lpVtbl->remove_ManipulationCompleted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_ManipulationCanceled(This, handler, token) \
    ((This)->lpVtbl->add_ManipulationCanceled(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_ManipulationCanceled(This, token) \
    ((This)->lpVtbl->remove_ManipulationCanceled(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_NavigationStarted(This, handler, token) \
    ((This)->lpVtbl->add_NavigationStarted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_NavigationStarted(This, token) \
    ((This)->lpVtbl->remove_NavigationStarted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_NavigationUpdated(This, handler, token) \
    ((This)->lpVtbl->add_NavigationUpdated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_NavigationUpdated(This, token) \
    ((This)->lpVtbl->remove_NavigationUpdated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_NavigationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_NavigationCompleted(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_NavigationCompleted(This, token) \
    ((This)->lpVtbl->remove_NavigationCompleted(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_add_NavigationCanceled(This, handler, token) \
    ((This)->lpVtbl->add_NavigationCanceled(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_remove_NavigationCanceled(This, token) \
    ((This)->lpVtbl->remove_NavigationCanceled(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_CaptureInteraction(This, interaction) \
    ((This)->lpVtbl->CaptureInteraction(This, interaction))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_CancelPendingGestures(This) \
    ((This)->lpVtbl->CancelPendingGestures(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_TrySetGestureSettings(This, settings, succeeded) \
    ((This)->lpVtbl->TrySetGestureSettings(This, settings, succeeded))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_get_GestureSettings(This, value) \
    ((This)->lpVtbl->get_GestureSettings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialGestureRecognizerFactory[] = L"Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings settings,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizer** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_Create(This, settings, value) \
    ((This)->lpVtbl->Create(This, settings, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialGestureRecognizerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialHoldStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialHoldStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteraction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteraction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteraction[] = L"Windows.UI.Input.Spatial.ISpatialInteraction";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceState)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_get_SourceState(This, value) \
    ((This)->lpVtbl->get_SourceState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasTouchpad)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasThumbstick)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);
    HRESULT (STDMETHODCALLTYPE* get_VendorId)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_HasTouchpad(This, value) \
    ((This)->lpVtbl->get_HasTouchpad(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_HasThumbstick(This, value) \
    ((This)->lpVtbl->get_HasThumbstick(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_VendorId(This, value) \
    ((This)->lpVtbl->get_VendorId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetRenderableModelAsync)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_TryGetRenderableModelAsync(This, value) \
    ((This)->lpVtbl->TryGetRenderableModelAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionController2
 *     Windows.UI.Input.Spatial.ISpatialInteractionController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionController3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionController3";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetBatteryReport)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3* This,
        __x_ABI_CWindows_CDevices_CPower_CIBatteryReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_TryGetBatteryReport(This, value) \
    ((This)->lpVtbl->TryGetBatteryReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionControllerProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionControllerProperties[] = L"Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTouchpadTouched)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTouchpadPressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsThumbstickPressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbstickX)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbstickY)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_TouchpadX)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_TouchpadY)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerPropertiesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_IsTouchpadTouched(This, value) \
    ((This)->lpVtbl->get_IsTouchpadTouched(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_IsTouchpadPressed(This, value) \
    ((This)->lpVtbl->get_IsTouchpadPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_IsThumbstickPressed(This, value) \
    ((This)->lpVtbl->get_IsThumbstickPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_ThumbstickX(This, value) \
    ((This)->lpVtbl->get_ThumbstickX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_ThumbstickY(This, value) \
    ((This)->lpVtbl->get_ThumbstickY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_TouchpadX(This, value) \
    ((This)->lpVtbl->get_TouchpadX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_get_TouchpadY(This, value) \
    ((This)->lpVtbl->get_TouchpadY(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionDetectedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);
    HRESULT (STDMETHODCALLTYPE* get_Interaction)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteraction** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_get_Interaction(This, value) \
    ((This)->lpVtbl->get_Interaction(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionDetectedEventArgs2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSource)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_get_InteractionSource(This, value) \
    ((This)->lpVtbl->get_InteractionSource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionDetectedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManager[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManager";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SourceDetected)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourceDetected)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SourceLost)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourceLost)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SourceUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourceUpdated)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SourcePressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourcePressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SourceReleased)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourceReleased)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_InteractionDetected)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CSpatial__CSpatialInteractionManager_Windows__CUI__CInput__CSpatial__CSpatialInteractionDetectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InteractionDetected)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetDetectedSourcesAtTimestamp)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timeStamp,
        __FIVectorView_1_Windows__CUI__CInput__CSpatial__CSpatialInteractionSourceState** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_SourceDetected(This, handler, token) \
    ((This)->lpVtbl->add_SourceDetected(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_SourceDetected(This, token) \
    ((This)->lpVtbl->remove_SourceDetected(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_SourceLost(This, handler, token) \
    ((This)->lpVtbl->add_SourceLost(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_SourceLost(This, token) \
    ((This)->lpVtbl->remove_SourceLost(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_SourceUpdated(This, handler, token) \
    ((This)->lpVtbl->add_SourceUpdated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_SourceUpdated(This, token) \
    ((This)->lpVtbl->remove_SourceUpdated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_SourcePressed(This, handler, token) \
    ((This)->lpVtbl->add_SourcePressed(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_SourcePressed(This, token) \
    ((This)->lpVtbl->remove_SourcePressed(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_SourceReleased(This, handler, token) \
    ((This)->lpVtbl->add_SourceReleased(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_SourceReleased(This, token) \
    ((This)->lpVtbl->remove_SourceReleased(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_add_InteractionDetected(This, handler, token) \
    ((This)->lpVtbl->add_InteractionDetected(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_remove_InteractionDetected(This, token) \
    ((This)->lpVtbl->remove_InteractionDetected(This, token))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_GetDetectedSourcesAtTimestamp(This, timeStamp, value) \
    ((This)->lpVtbl->GetDetectedSourcesAtTimestamp(This, timeStamp, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManagerStatics[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionManagerStatics2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSourceKindSupported)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind kind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_IsSourceKindSupported(This, kind, result) \
    ((This)->lpVtbl->IsSourceKindSupported(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPointingSupported)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMenuSupported)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGraspSupported)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Controller)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionController** value);
    HRESULT (STDMETHODCALLTYPE* TryGetStateAtTimestamp)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timestamp,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_get_IsPointingSupported(This, value) \
    ((This)->lpVtbl->get_IsPointingSupported(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_get_IsMenuSupported(This, value) \
    ((This)->lpVtbl->get_IsMenuSupported(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_get_IsGraspSupported(This, value) \
    ((This)->lpVtbl->get_IsGraspSupported(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_get_Controller(This, value) \
    ((This)->lpVtbl->get_Controller(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_TryGetStateAtTimestamp(This, timestamp, value) \
    ((This)->lpVtbl->TryGetStateAtTimestamp(This, timestamp, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *     Windows.UI.Input.Spatial.ISpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource3";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handedness)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceHandedness* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_get_Handedness(This, value) \
    ((This)->lpVtbl->get_Handedness(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSource4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSource4[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSource4";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateHandMeshObserver)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver** result);
    HRESULT (STDMETHODCALLTYPE* TryCreateHandMeshObserverAsync)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4* This,
        __FIAsyncOperation_1_Windows__CPerception__CPeople__CHandMeshObserver** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_TryCreateHandMeshObserver(This, result) \
    ((This)->lpVtbl->TryCreateHandMeshObserver(This, result))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_TryCreateHandMeshObserverAsync(This, operation) \
    ((This)->lpVtbl->TryCreateHandMeshObserverAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceEventArgs2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PressKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionPressKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_get_PressKind(This, value) \
    ((This)->lpVtbl->get_PressKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value);
    HRESULT (STDMETHODCALLTYPE* get_Velocity)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation* This,
        __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocationVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_get_Velocity(This, value) \
    ((This)->lpVtbl->get_Velocity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2* This,
        __FIReference_1_Windows__CFoundation__CNumerics__CQuaternion** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceLocation3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionAccuracy)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourcePositionAccuracy* value);
    HRESULT (STDMETHODCALLTYPE* get_AngularVelocity)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_get_PositionAccuracy(This, value) \
    ((This)->lpVtbl->get_PositionAccuracy(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_get_AngularVelocity(This, value) \
    ((This)->lpVtbl->get_AngularVelocity(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_get_SourcePointerPose(This, value) \
    ((This)->lpVtbl->get_SourcePointerPose(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceProperties[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourcePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetSourceLossMitigationDirection)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __FIReference_1_Windows__CFoundation__CNumerics__CVector3** value);
    HRESULT (STDMETHODCALLTYPE* get_SourceLossRisk)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* TryGetLocation)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceLocation** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourcePropertiesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourcePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_TryGetSourceLossMitigationDirection(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetSourceLossMitigationDirection(This, coordinateSystem, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_get_SourceLossRisk(This, value) \
    ((This)->lpVtbl->get_SourceLossRisk(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_TryGetLocation(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetLocation(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_IsPressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp** value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceStateVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_get_IsPressed(This, value) \
    ((This)->lpVtbl->get_IsPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState2[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSelectPressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMenuPressed)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGrasped)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectPressedValue)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_ControllerProperties)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionControllerProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_get_IsSelectPressed(This, value) \
    ((This)->lpVtbl->get_IsSelectPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_get_IsMenuPressed(This, value) \
    ((This)->lpVtbl->get_IsMenuPressed(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_get_IsGrasped(This, value) \
    ((This)->lpVtbl->get_IsGrasped(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_get_SelectPressedValue(This, value) \
    ((This)->lpVtbl->get_SelectPressedValue(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_get_ControllerProperties(This, value) \
    ((This)->lpVtbl->get_ControllerProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialInteractionSourceState3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *     Windows.UI.Input.Spatial.ISpatialInteractionSourceState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialInteractionSourceState3[] = L"Windows.UI.Input.Spatial.ISpatialInteractionSourceState3";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetHandPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHandPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_TryGetHandPose(This, value) \
    ((This)->lpVtbl->TryGetHandPose(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSourceState3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetCumulativeDelta)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_TryGetCumulativeDelta(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetCumulativeDelta(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationDelta
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationDelta[] = L"Windows.UI.Input.Spatial.ISpatialManipulationDelta";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDeltaVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Translation)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDeltaVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDeltaVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_get_Translation(This, value) \
    ((This)->lpVtbl->get_Translation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialManipulationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetCumulativeDelta)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationDelta** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_TryGetCumulativeDelta(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetCumulativeDelta(This, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialManipulationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCanceledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* get_NormalizedOffset)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_get_NormalizedOffset(This, value) \
    ((This)->lpVtbl->get_NormalizedOffset(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);
    HRESULT (STDMETHODCALLTYPE* get_IsNavigatingX)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsNavigatingY)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsNavigatingZ)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_get_IsNavigatingX(This, value) \
    ((This)->lpVtbl->get_IsNavigatingX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_get_IsNavigatingY(This, value) \
    ((This)->lpVtbl->get_IsNavigatingY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_get_IsNavigatingZ(This, value) \
    ((This)->lpVtbl->get_IsNavigatingZ(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialNavigationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* get_NormalizedOffset)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_get_NormalizedOffset(This, value) \
    ((This)->lpVtbl->get_NormalizedOffset(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialNavigationUpdatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerInteractionSourcePose[] = L"Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_ForwardDirection)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_UpDirection)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePoseVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_get_ForwardDirection(This, value) \
    ((This)->lpVtbl->get_ForwardDirection(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_get_UpDirection(This, value) \
    ((This)->lpVtbl->get_UpDirection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerInteractionSourcePose2[] = L"Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* get_PositionAccuracy)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourcePositionAccuracy* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_get_PositionAccuracy(This, value) \
    ((This)->lpVtbl->get_PositionAccuracy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp** value);
    HRESULT (STDMETHODCALLTYPE* get_Head)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHeadPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_get_Head(This, value) \
    ((This)->lpVtbl->get_Head(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Spatial.ISpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose2[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose2";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetInteractionSourcePose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource* source,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerInteractionSourcePose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_TryGetInteractionSourcePose(This, source, value) \
    ((This)->lpVtbl->TryGetInteractionSourcePose(This, source, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPose3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPose3[] = L"Windows.UI.Input.Spatial.ISpatialPointerPose3";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Eyes)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        __x_ABI_CWindows_CPerception_CPeople_CIEyesPose** value);
    HRESULT (STDMETHODCALLTYPE* get_IsHeadCapturedBySystem)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_get_Eyes(This, value) \
    ((This)->lpVtbl->get_Eyes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_get_IsHeadCapturedBySystem(This, value) \
    ((This)->lpVtbl->get_IsHeadCapturedBySystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialPointerPoseStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialPointerPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialPointerPoseStatics[] = L"Windows.UI.Input.Spatial.ISpatialPointerPoseStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetAtTimestamp)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp* timestamp,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_TryGetAtTimestamp(This, coordinateSystem, timestamp, value) \
    ((This)->lpVtbl->TryGetAtTimestamp(This, coordinateSystem, timestamp, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPoseStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialRecognitionEndedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionEndedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialRecognitionStartedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);
    HRESULT (STDMETHODCALLTYPE* IsGesturePossible)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialGestureSettings gesture,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_IsGesturePossible(This, gesture, value) \
    ((This)->lpVtbl->IsGesturePossible(This, gesture, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialRecognitionStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Input.Spatial.ISpatialTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Spatial.SpatialTappedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Spatial_ISpatialTappedEventArgs[] = L"Windows.UI.Input.Spatial.ISpatialTappedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InteractionSourceKind)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CSpatial_CSpatialInteractionSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* TryGetPointerPose)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialPointerPose** value);
    HRESULT (STDMETHODCALLTYPE* get_TapCount)(__x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_get_InteractionSourceKind(This, value) \
    ((This)->lpVtbl->get_InteractionSourceKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_TryGetPointerPose(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetPointerPose(This, coordinateSystem, value))

#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_get_TapCount(This, value) \
    ((This)->lpVtbl->get_TapCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialTappedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialGestureRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialGestureRecognizer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialGestureRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialGestureRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialGestureRecognizer[] = L"Windows.UI.Input.Spatial.SpatialGestureRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialHoldStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteraction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteraction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteraction_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteraction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteraction[] = L"Windows.UI.Input.Spatial.SpatialInteraction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionController ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionController2
 *    Windows.UI.Input.Spatial.ISpatialInteractionController3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionController_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionController[] = L"Windows.UI.Input.Spatial.SpatialInteractionController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionControllerProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionControllerProperties[] = L"Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionDetectedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionManager[] = L"Windows.UI.Input.Spatial.SpatialInteractionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource3
 *    Windows.UI.Input.Spatial.ISpatialInteractionSource4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSource[] = L"Windows.UI.Input.Spatial.SpatialInteractionSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceEventArgs[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceLocation[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceProperties[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialInteractionSourceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState2
 *    Windows.UI.Input.Spatial.ISpatialInteractionSourceState3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceState_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialInteractionSourceState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialInteractionSourceState[] = L"Windows.UI.Input.Spatial.SpatialInteractionSourceState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationDelta ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationDelta_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationDelta_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationDelta[] = L"Windows.UI.Input.Spatial.SpatialManipulationDelta";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialManipulationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationCanceledEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationCompletedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialNavigationUpdatedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialPointerInteractionSourcePose[] = L"Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialPointerPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Spatial.ISpatialPointerPoseStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialPointerPose ** Default Interface **
 *    Windows.UI.Input.Spatial.ISpatialPointerPose2
 *    Windows.UI.Input.Spatial.ISpatialPointerPose3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerPose_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialPointerPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialPointerPose[] = L"Windows.UI.Input.Spatial.SpatialPointerPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialRecognitionEndedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialRecognitionStartedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.Input.Spatial.SpatialTappedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Spatial.ISpatialTappedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialTappedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Spatial_SpatialTappedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Spatial_SpatialTappedEventArgs[] = L"Windows.UI.Input.Spatial.SpatialTappedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput2Espatial_p_h__

#endif // __windows2Eui2Einput2Espatial_h__
