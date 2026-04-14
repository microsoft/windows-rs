
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
#ifndef __windows2Egaming2Einput_h__
#define __windows2Egaming2Einput_h__
#ifndef __windows2Egaming2Einput_p_h__
#define __windows2Egaming2Einput_p_h__


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

#if !defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)
#define WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)

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
#include "Windows.Devices.Haptics.h"
#include "Windows.Devices.Power.h"
#include "Windows.Gaming.Input.ForceFeedback.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IArcadeStick;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick ABI::Windows::Gaming::Input::IArcadeStick

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IArcadeStickStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics ABI::Windows::Gaming::Input::IArcadeStickStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IArcadeStickStatics2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2 ABI::Windows::Gaming::Input::IArcadeStickStatics2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IFlightStick;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick ABI::Windows::Gaming::Input::IFlightStick

#endif // ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IFlightStickStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics ABI::Windows::Gaming::Input::IFlightStickStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGameController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGameController ABI::Windows::Gaming::Input::IGameController

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGameControllerBatteryInfo;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo ABI::Windows::Gaming::Input::IGameControllerBatteryInfo

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGamepad;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGamepad ABI::Windows::Gaming::Input::IGamepad

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGamepad2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2 ABI::Windows::Gaming::Input::IGamepad2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGamepadStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics ABI::Windows::Gaming::Input::IGamepadStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGamepadStatics2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2 ABI::Windows::Gaming::Input::IGamepadStatics2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IHeadset;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIHeadset ABI::Windows::Gaming::Input::IHeadset

#endif // ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRacingWheel;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel ABI::Windows::Gaming::Input::IRacingWheel

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRacingWheelStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics ABI::Windows::Gaming::Input::IRacingWheelStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRacingWheelStatics2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2 ABI::Windows::Gaming::Input::IRacingWheelStatics2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRawGameController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController ABI::Windows::Gaming::Input::IRawGameController

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRawGameController2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2 ABI::Windows::Gaming::Input::IRawGameController2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IRawGameControllerStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics ABI::Windows::Gaming::Input::IRawGameControllerStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IUINavigationController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController ABI::Windows::Gaming::Input::IUINavigationController

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IUINavigationControllerStatics;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics ABI::Windows::Gaming::Input::IUINavigationControllerStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IUINavigationControllerStatics2;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2 ABI::Windows::Gaming::Input::IUINavigationControllerStatics2

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#define DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3c501ba4-eda4-5238-bdb7-d10ba350cd83"))
IIterator<ABI::Windows::Devices::Haptics::SimpleHapticsController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsController*, ABI::Windows::Devices::Haptics::ISimpleHapticsController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Haptics.SimpleHapticsController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Haptics::SimpleHapticsController*> __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t;
#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#define DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b50da692-4a2b-5c8a-8e14-0439c0b1dba4"))
IIterable<ABI::Windows::Devices::Haptics::SimpleHapticsController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsController*, ABI::Windows::Devices::Haptics::ISimpleHapticsController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Haptics.SimpleHapticsController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Haptics::SimpleHapticsController*> __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t;
#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class ArcadeStick;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CArcadeStick_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CArcadeStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d30629af-cc9d-52e1-8b1f-0ffa9629afee"))
IIterator<ABI::Windows::Gaming::Input::ArcadeStick*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ArcadeStick*, ABI::Windows::Gaming::Input::IArcadeStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.ArcadeStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::ArcadeStick*> __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_t;
#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CArcadeStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CArcadeStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CArcadeStick_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CArcadeStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9376f457-2da5-544a-a409-c636f5d81c35"))
IIterable<ABI::Windows::Gaming::Input::ArcadeStick*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ArcadeStick*, ABI::Windows::Gaming::Input::IArcadeStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.ArcadeStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::ArcadeStick*> __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_t;
#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CArcadeStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CArcadeStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class FlightStick;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CFlightStick_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CFlightStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f5fa1919-3f18-5560-bb13-cf7018ac41d5"))
IIterator<ABI::Windows::Gaming::Input::FlightStick*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::FlightStick*, ABI::Windows::Gaming::Input::IFlightStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.FlightStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::FlightStick*> __FIIterator_1_Windows__CGaming__CInput__CFlightStick_t;
#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CFlightStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CFlightStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CFlightStick_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CFlightStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3b7fc175-bebe-52ef-a3e9-dda75ea1acfc"))
IIterable<ABI::Windows::Gaming::Input::FlightStick*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::FlightStick*, ABI::Windows::Gaming::Input::IFlightStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.FlightStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::FlightStick*> __FIIterable_1_Windows__CGaming__CInput__CFlightStick_t;
#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CFlightStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CFlightStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    class ForceFeedbackMotor;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IForceFeedbackMotor;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64cf69e0-5464-5b72-bd4b-82f7c3d0386d"))
IIterator<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*, ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t;
#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c14440d1-fea0-5147-aed8-9b85239da882"))
IIterable<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*, ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t;
#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class Gamepad;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CGamepad_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CGamepad_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("246737e8-12bc-5c64-af52-06db4b13fa2f"))
IIterator<ABI::Windows::Gaming::Input::Gamepad*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Gamepad*, ABI::Windows::Gaming::Input::IGamepad*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.Gamepad>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::Gamepad*> __FIIterator_1_Windows__CGaming__CInput__CGamepad_t;
#define __FIIterator_1_Windows__CGaming__CInput__CGamepad ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CGamepad_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CGamepad_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CGamepad_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CGamepad_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("47132ba0-6b17-5cd2-a8bd-b5d3443ccb13"))
IIterable<ABI::Windows::Gaming::Input::Gamepad*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Gamepad*, ABI::Windows::Gaming::Input::IGamepad*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.Gamepad>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::Gamepad*> __FIIterable_1_Windows__CGaming__CInput__CGamepad_t;
#define __FIIterable_1_Windows__CGaming__CInput__CGamepad ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CGamepad_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CGamepad_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class RacingWheel;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CRacingWheel_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CRacingWheel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23d735b8-4d36-5377-a245-69df97c9fcd9"))
IIterator<ABI::Windows::Gaming::Input::RacingWheel*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RacingWheel*, ABI::Windows::Gaming::Input::IRacingWheel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.RacingWheel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::RacingWheel*> __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_t;
#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CRacingWheel_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CRacingWheel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CRacingWheel_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CRacingWheel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9a7c3830-9a87-5287-a1e2-8a2af29cf68c"))
IIterable<ABI::Windows::Gaming::Input::RacingWheel*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RacingWheel*, ABI::Windows::Gaming::Input::IRacingWheel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.RacingWheel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::RacingWheel*> __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_t;
#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CRacingWheel_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CRacingWheel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class RawGameController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CRawGameController_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CRawGameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("51cc88dc-66fb-55ea-9a1b-aadcd71cc08e"))
IIterator<ABI::Windows::Gaming::Input::RawGameController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RawGameController*, ABI::Windows::Gaming::Input::IRawGameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.RawGameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::RawGameController*> __FIIterator_1_Windows__CGaming__CInput__CRawGameController_t;
#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CRawGameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CRawGameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CRawGameController_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CRawGameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8f2f08cc-f4f4-5539-9357-1f07334d381f"))
IIterable<ABI::Windows::Gaming::Input::RawGameController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RawGameController*, ABI::Windows::Gaming::Input::IRawGameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.RawGameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::RawGameController*> __FIIterable_1_Windows__CGaming__CInput__CRawGameController_t;
#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CRawGameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CRawGameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class UINavigationController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CGaming__CInput__CUINavigationController_USE
#define DEF___FIIterator_1_Windows__CGaming__CInput__CUINavigationController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c10b2696-64a3-5262-bc4f-b741e5d5afab"))
IIterator<ABI::Windows::Gaming::Input::UINavigationController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::UINavigationController*, ABI::Windows::Gaming::Input::IUINavigationController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Input.UINavigationController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Input::UINavigationController*> __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_t;
#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CInput__CUINavigationController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CInput__CUINavigationController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CGaming__CInput__CUINavigationController_USE
#define DEF___FIIterable_1_Windows__CGaming__CInput__CUINavigationController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8dea85a0-0204-57dd-abad-90e37c0ef240"))
IIterable<ABI::Windows::Gaming::Input::UINavigationController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::UINavigationController*, ABI::Windows::Gaming::Input::IUINavigationController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Input.UINavigationController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Input::UINavigationController*> __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_t;
#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CInput__CUINavigationController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CInput__CUINavigationController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5390f01e-c701-5382-97cc-94eaac4b6cbf"))
IVectorView<ABI::Windows::Devices::Haptics::SimpleHapticsController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsController*, ABI::Windows::Devices::Haptics::ISimpleHapticsController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Haptics.SimpleHapticsController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Haptics::SimpleHapticsController*> __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t;
#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("becace75-d0cd-5a9c-845f-72f085503cdf"))
IVectorView<ABI::Windows::Gaming::Input::ArcadeStick*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ArcadeStick*, ABI::Windows::Gaming::Input::IArcadeStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.ArcadeStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::ArcadeStick*> __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CFlightStick_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CFlightStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8b9d067e-b6f5-592f-a90a-d72c3d98d4da"))
IVectorView<ABI::Windows::Gaming::Input::FlightStick*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::FlightStick*, ABI::Windows::Gaming::Input::IFlightStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.FlightStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::FlightStick*> __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CFlightStick_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CFlightStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5bfc5070-101d-5fbb-8d5f-ce5c23becdd9"))
IVectorView<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*, ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackMotor*> __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CGamepad_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CGamepad_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb97bb69-09c9-5a99-86b2-3e36085284d4"))
IVectorView<ABI::Windows::Gaming::Input::Gamepad*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Gamepad*, ABI::Windows::Gaming::Input::IGamepad*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.Gamepad>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::Gamepad*> __FIVectorView_1_Windows__CGaming__CInput__CGamepad_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CGamepad_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CGamepad_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("153993b2-6052-5959-91ec-900c53fef120"))
IVectorView<ABI::Windows::Gaming::Input::RacingWheel*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RacingWheel*, ABI::Windows::Gaming::Input::IRacingWheel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.RacingWheel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::RacingWheel*> __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CRawGameController_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CRawGameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("779cc322-40c0-55c1-8dc5-cc6e3afe02cf"))
IVectorView<ABI::Windows::Gaming::Input::RawGameController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RawGameController*, ABI::Windows::Gaming::Input::IRawGameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.RawGameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::RawGameController*> __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CRawGameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CRawGameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_USE
#define DEF___FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("684b9e5a-2dc2-54fc-adf5-5ca8f6871425"))
IVectorView<ABI::Windows::Gaming::Input::UINavigationController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::UINavigationController*, ABI::Windows::Gaming::Input::IUINavigationController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Input.UINavigationController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Input::UINavigationController*> __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_t;
#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6afb8188-d28d-539b-bb69-ea1763fb9920"))
IEventHandler<ABI::Windows::Gaming::Input::ArcadeStick*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::ArcadeStick*, ABI::Windows::Gaming::Input::IArcadeStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.ArcadeStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::ArcadeStick*> __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d57470b1-cc22-5a43-8e18-5ca064aafe21"))
IEventHandler<ABI::Windows::Gaming::Input::FlightStick*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::FlightStick*, ABI::Windows::Gaming::Input::IFlightStick*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.FlightStick>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::FlightStick*> __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CGamepad_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CGamepad_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a7639ee-624a-501a-bb53-562d1ec11b52"))
IEventHandler<ABI::Windows::Gaming::Input::Gamepad*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Gamepad*, ABI::Windows::Gaming::Input::IGamepad*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.Gamepad>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::Gamepad*> __FIEventHandler_1_Windows__CGaming__CInput__CGamepad_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CGamepad ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CGamepad_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CGamepad_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("352ec824-f64b-5353-80ea-7ff58e3b92a4"))
IEventHandler<ABI::Windows::Gaming::Input::RacingWheel*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RacingWheel*, ABI::Windows::Gaming::Input::IRacingWheel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.RacingWheel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::RacingWheel*> __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("00621c22-42e8-529f-9270-836b32931d72"))
IEventHandler<ABI::Windows::Gaming::Input::RawGameController*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::RawGameController*, ABI::Windows::Gaming::Input::IRawGameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.RawGameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::RawGameController*> __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_USE
#define DEF___FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9eaec424-75c1-5871-8da9-ce590c653045"))
IEventHandler<ABI::Windows::Gaming::Input::UINavigationController*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::UINavigationController*, ABI::Windows::Gaming::Input::IUINavigationController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Gaming.Input.UINavigationController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Gaming::Input::UINavigationController*> __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_t;
#define __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController ABI::Windows::Foundation::__FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                class Headset;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_USE
#define DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07b2f2b7-8825-5c4e-a052-fcfedf3aeea1"))
ITypedEventHandler<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::Gaming::Input::Headset*> : ITypedEventHandler_impl<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Headset*, ABI::Windows::Gaming::Input::IHeadset*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Gaming.Input.IGameController, Windows.Gaming.Input.Headset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::Gaming::Input::Headset*> __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_t;
#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            class UserChangedEventArgs;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUserChangedEventArgs;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUserChangedEventArgs ABI::Windows::System::IUserChangedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb753f2c-2f36-5a8f-adad-057beae73aa4"))
ITypedEventHandler<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::System::UserChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::UserChangedEventArgs*, ABI::Windows::System::IUserChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Gaming.Input.IGameController, Windows.System.UserChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Gaming::Input::IGameController*, ABI::Windows::System::UserChangedEventArgs*> __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum ArcadeStickButtons : unsigned int ArcadeStickButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum FlightStickButtons : unsigned int FlightStickButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum GameControllerButtonLabel : int GameControllerButtonLabel;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum GameControllerSwitchKind : int GameControllerSwitchKind;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum GameControllerSwitchPosition : int GameControllerSwitchPosition;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum GamepadButtons : unsigned int GamepadButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum OptionalUINavigationButtons : unsigned int OptionalUINavigationButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum RacingWheelButtons : unsigned int RacingWheelButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum RequiredUINavigationButtons : unsigned int RequiredUINavigationButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct ArcadeStickReading ArcadeStickReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct FlightStickReading FlightStickReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct GamepadReading GamepadReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct GamepadVibration GamepadVibration;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct RacingWheelReading RacingWheelReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct UINavigationReading UINavigationReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.Input.ArcadeStickButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum ArcadeStickButtons : unsigned int
                {
                    ArcadeStickButtons_None = 0,
                    ArcadeStickButtons_StickUp = 0x1,
                    ArcadeStickButtons_StickDown = 0x2,
                    ArcadeStickButtons_StickLeft = 0x4,
                    ArcadeStickButtons_StickRight = 0x8,
                    ArcadeStickButtons_Action1 = 0x10,
                    ArcadeStickButtons_Action2 = 0x20,
                    ArcadeStickButtons_Action3 = 0x40,
                    ArcadeStickButtons_Action4 = 0x80,
                    ArcadeStickButtons_Action5 = 0x100,
                    ArcadeStickButtons_Action6 = 0x200,
                    ArcadeStickButtons_Special1 = 0x400,
                    ArcadeStickButtons_Special2 = 0x800,
                };

                DEFINE_ENUM_FLAG_OPERATORS(ArcadeStickButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.FlightStickButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum FlightStickButtons : unsigned int
                {
                    FlightStickButtons_None = 0,
                    FlightStickButtons_FirePrimary = 0x1,
                    FlightStickButtons_FireSecondary = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(FlightStickButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerButtonLabel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum GameControllerButtonLabel : int
                {
                    GameControllerButtonLabel_None = 0,
                    GameControllerButtonLabel_XboxBack = 1,
                    GameControllerButtonLabel_XboxStart = 2,
                    GameControllerButtonLabel_XboxMenu = 3,
                    GameControllerButtonLabel_XboxView = 4,
                    GameControllerButtonLabel_XboxUp = 5,
                    GameControllerButtonLabel_XboxDown = 6,
                    GameControllerButtonLabel_XboxLeft = 7,
                    GameControllerButtonLabel_XboxRight = 8,
                    GameControllerButtonLabel_XboxA = 9,
                    GameControllerButtonLabel_XboxB = 10,
                    GameControllerButtonLabel_XboxX = 11,
                    GameControllerButtonLabel_XboxY = 12,
                    GameControllerButtonLabel_XboxLeftBumper = 13,
                    GameControllerButtonLabel_XboxLeftTrigger = 14,
                    GameControllerButtonLabel_XboxLeftStickButton = 15,
                    GameControllerButtonLabel_XboxRightBumper = 16,
                    GameControllerButtonLabel_XboxRightTrigger = 17,
                    GameControllerButtonLabel_XboxRightStickButton = 18,
                    GameControllerButtonLabel_XboxPaddle1 = 19,
                    GameControllerButtonLabel_XboxPaddle2 = 20,
                    GameControllerButtonLabel_XboxPaddle3 = 21,
                    GameControllerButtonLabel_XboxPaddle4 = 22,
                    GameControllerButtonLabel_Mode = 23,
                    GameControllerButtonLabel_Select = 24,
                    GameControllerButtonLabel_Menu = 25,
                    GameControllerButtonLabel_View = 26,
                    GameControllerButtonLabel_Back = 27,
                    GameControllerButtonLabel_Start = 28,
                    GameControllerButtonLabel_Options = 29,
                    GameControllerButtonLabel_Share = 30,
                    GameControllerButtonLabel_Up = 31,
                    GameControllerButtonLabel_Down = 32,
                    GameControllerButtonLabel_Left = 33,
                    GameControllerButtonLabel_Right = 34,
                    GameControllerButtonLabel_LetterA = 35,
                    GameControllerButtonLabel_LetterB = 36,
                    GameControllerButtonLabel_LetterC = 37,
                    GameControllerButtonLabel_LetterL = 38,
                    GameControllerButtonLabel_LetterR = 39,
                    GameControllerButtonLabel_LetterX = 40,
                    GameControllerButtonLabel_LetterY = 41,
                    GameControllerButtonLabel_LetterZ = 42,
                    GameControllerButtonLabel_Cross = 43,
                    GameControllerButtonLabel_Circle = 44,
                    GameControllerButtonLabel_Square = 45,
                    GameControllerButtonLabel_Triangle = 46,
                    GameControllerButtonLabel_LeftBumper = 47,
                    GameControllerButtonLabel_LeftTrigger = 48,
                    GameControllerButtonLabel_LeftStickButton = 49,
                    GameControllerButtonLabel_Left1 = 50,
                    GameControllerButtonLabel_Left2 = 51,
                    GameControllerButtonLabel_Left3 = 52,
                    GameControllerButtonLabel_RightBumper = 53,
                    GameControllerButtonLabel_RightTrigger = 54,
                    GameControllerButtonLabel_RightStickButton = 55,
                    GameControllerButtonLabel_Right1 = 56,
                    GameControllerButtonLabel_Right2 = 57,
                    GameControllerButtonLabel_Right3 = 58,
                    GameControllerButtonLabel_Paddle1 = 59,
                    GameControllerButtonLabel_Paddle2 = 60,
                    GameControllerButtonLabel_Paddle3 = 61,
                    GameControllerButtonLabel_Paddle4 = 62,
                    GameControllerButtonLabel_Plus = 63,
                    GameControllerButtonLabel_Minus = 64,
                    GameControllerButtonLabel_DownLeftArrow = 65,
                    GameControllerButtonLabel_DialLeft = 66,
                    GameControllerButtonLabel_DialRight = 67,
                    GameControllerButtonLabel_Suspension = 68,
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerSwitchKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum GameControllerSwitchKind : int
                {
                    GameControllerSwitchKind_TwoWay = 0,
                    GameControllerSwitchKind_FourWay = 1,
                    GameControllerSwitchKind_EightWay = 2,
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerSwitchPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum GameControllerSwitchPosition : int
                {
                    GameControllerSwitchPosition_Center = 0,
                    GameControllerSwitchPosition_Up = 1,
                    GameControllerSwitchPosition_UpRight = 2,
                    GameControllerSwitchPosition_Right = 3,
                    GameControllerSwitchPosition_DownRight = 4,
                    GameControllerSwitchPosition_Down = 5,
                    GameControllerSwitchPosition_DownLeft = 6,
                    GameControllerSwitchPosition_Left = 7,
                    GameControllerSwitchPosition_UpLeft = 8,
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GamepadButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum GamepadButtons : unsigned int
                {
                    GamepadButtons_None = 0,
                    GamepadButtons_Menu = 0x1,
                    GamepadButtons_View = 0x2,
                    GamepadButtons_A = 0x4,
                    GamepadButtons_B = 0x8,
                    GamepadButtons_X = 0x10,
                    GamepadButtons_Y = 0x20,
                    GamepadButtons_DPadUp = 0x40,
                    GamepadButtons_DPadDown = 0x80,
                    GamepadButtons_DPadLeft = 0x100,
                    GamepadButtons_DPadRight = 0x200,
                    GamepadButtons_LeftShoulder = 0x400,
                    GamepadButtons_RightShoulder = 0x800,
                    GamepadButtons_LeftThumbstick = 0x1000,
                    GamepadButtons_RightThumbstick = 0x2000,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    GamepadButtons_Paddle1 = 0x4000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    GamepadButtons_Paddle2 = 0x8000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    GamepadButtons_Paddle3 = 0x10000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    GamepadButtons_Paddle4 = 0x20000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };

                DEFINE_ENUM_FLAG_OPERATORS(GamepadButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.OptionalUINavigationButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum OptionalUINavigationButtons : unsigned int
                {
                    OptionalUINavigationButtons_None = 0,
                    OptionalUINavigationButtons_Context1 = 0x1,
                    OptionalUINavigationButtons_Context2 = 0x2,
                    OptionalUINavigationButtons_Context3 = 0x4,
                    OptionalUINavigationButtons_Context4 = 0x8,
                    OptionalUINavigationButtons_PageUp = 0x10,
                    OptionalUINavigationButtons_PageDown = 0x20,
                    OptionalUINavigationButtons_PageLeft = 0x40,
                    OptionalUINavigationButtons_PageRight = 0x80,
                    OptionalUINavigationButtons_ScrollUp = 0x100,
                    OptionalUINavigationButtons_ScrollDown = 0x200,
                    OptionalUINavigationButtons_ScrollLeft = 0x400,
                    OptionalUINavigationButtons_ScrollRight = 0x800,
                };

                DEFINE_ENUM_FLAG_OPERATORS(OptionalUINavigationButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.RacingWheelButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum RacingWheelButtons : unsigned int
                {
                    RacingWheelButtons_None = 0,
                    RacingWheelButtons_PreviousGear = 0x1,
                    RacingWheelButtons_NextGear = 0x2,
                    RacingWheelButtons_DPadUp = 0x4,
                    RacingWheelButtons_DPadDown = 0x8,
                    RacingWheelButtons_DPadLeft = 0x10,
                    RacingWheelButtons_DPadRight = 0x20,
                    RacingWheelButtons_Button1 = 0x40,
                    RacingWheelButtons_Button2 = 0x80,
                    RacingWheelButtons_Button3 = 0x100,
                    RacingWheelButtons_Button4 = 0x200,
                    RacingWheelButtons_Button5 = 0x400,
                    RacingWheelButtons_Button6 = 0x800,
                    RacingWheelButtons_Button7 = 0x1000,
                    RacingWheelButtons_Button8 = 0x2000,
                    RacingWheelButtons_Button9 = 0x4000,
                    RacingWheelButtons_Button10 = 0x8000,
                    RacingWheelButtons_Button11 = 0x10000,
                    RacingWheelButtons_Button12 = 0x20000,
                    RacingWheelButtons_Button13 = 0x40000,
                    RacingWheelButtons_Button14 = 0x80000,
                    RacingWheelButtons_Button15 = 0x100000,
                    RacingWheelButtons_Button16 = 0x200000,
                };

                DEFINE_ENUM_FLAG_OPERATORS(RacingWheelButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.RequiredUINavigationButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                enum RequiredUINavigationButtons : unsigned int
                {
                    RequiredUINavigationButtons_None = 0,
                    RequiredUINavigationButtons_Menu = 0x1,
                    RequiredUINavigationButtons_View = 0x2,
                    RequiredUINavigationButtons_Accept = 0x4,
                    RequiredUINavigationButtons_Cancel = 0x8,
                    RequiredUINavigationButtons_Up = 0x10,
                    RequiredUINavigationButtons_Down = 0x20,
                    RequiredUINavigationButtons_Left = 0x40,
                    RequiredUINavigationButtons_Right = 0x80,
                };

                DEFINE_ENUM_FLAG_OPERATORS(RequiredUINavigationButtons)
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ArcadeStickReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct ArcadeStickReading
                {
                    UINT64 Timestamp;
                    ABI::Windows::Gaming::Input::ArcadeStickButtons Buttons;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.FlightStickReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct FlightStickReading
                {
                    UINT64 Timestamp;
                    ABI::Windows::Gaming::Input::FlightStickButtons Buttons;
                    ABI::Windows::Gaming::Input::GameControllerSwitchPosition HatSwitch;
                    DOUBLE Roll;
                    DOUBLE Pitch;
                    DOUBLE Yaw;
                    DOUBLE Throttle;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GamepadReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct GamepadReading
                {
                    UINT64 Timestamp;
                    ABI::Windows::Gaming::Input::GamepadButtons Buttons;
                    DOUBLE LeftTrigger;
                    DOUBLE RightTrigger;
                    DOUBLE LeftThumbstickX;
                    DOUBLE LeftThumbstickY;
                    DOUBLE RightThumbstickX;
                    DOUBLE RightThumbstickY;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.GamepadVibration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct GamepadVibration
                {
                    DOUBLE LeftMotor;
                    DOUBLE RightMotor;
                    DOUBLE LeftTrigger;
                    DOUBLE RightTrigger;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.RacingWheelReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct RacingWheelReading
                {
                    UINT64 Timestamp;
                    ABI::Windows::Gaming::Input::RacingWheelButtons Buttons;
                    INT32 PatternShifterGear;
                    DOUBLE Wheel;
                    DOUBLE Throttle;
                    DOUBLE Brake;
                    DOUBLE Clutch;
                    DOUBLE Handbrake;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.UINavigationReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                struct UINavigationReading
                {
                    UINT64 Timestamp;
                    ABI::Windows::Gaming::Input::RequiredUINavigationButtons RequiredButtons;
                    ABI::Windows::Gaming::Input::OptionalUINavigationButtons OptionalButtons;
                };
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStick[] = L"Windows.Gaming.Input.IArcadeStick";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("b14a539d-befb-4c81-8051-15ecf3b13036")
                IArcadeStick : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetButtonLabel(
                        ABI::Windows::Gaming::Input::ArcadeStickButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        ABI::Windows::Gaming::Input::ArcadeStickReading* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IArcadeStick = __uuidof(IArcadeStick);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStick;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStickStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStickStatics[] = L"Windows.Gaming.Input.IArcadeStickStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("5c37b8c8-37b1-4ad8-9458-200f1a30018e")
                IArcadeStickStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ArcadeStickAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ArcadeStickAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ArcadeStickRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ArcadeStickRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ArcadeSticks(
                        __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IArcadeStickStatics = __uuidof(IArcadeStickStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStickStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IArcadeStickStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStickStatics2[] = L"Windows.Gaming.Input.IArcadeStickStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("52b5d744-bb86-445a-b59c-596f0e2a49df")
                IArcadeStickStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IArcadeStick** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IArcadeStickStatics2 = __uuidof(IArcadeStickStatics2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IFlightStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.FlightStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IFlightStick[] = L"Windows.Gaming.Input.IFlightStick";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("b4a2c01c-b83b-4459-a1a9-97b03c33da7c")
                IFlightStick : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HatSwitchKind(
                        ABI::Windows::Gaming::Input::GameControllerSwitchKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetButtonLabel(
                        ABI::Windows::Gaming::Input::FlightStickButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        ABI::Windows::Gaming::Input::FlightStickReading* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFlightStick = __uuidof(IFlightStick);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIFlightStick;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IFlightStickStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.FlightStick
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IFlightStickStatics[] = L"Windows.Gaming.Input.IFlightStickStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("5514924a-fecc-435e-83dc-5cec8a18a520")
                IFlightStickStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_FlightStickAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FlightStickAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FlightStickRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FlightStickRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FlightSticks(
                        __FIVectorView_1_Windows__CGaming__CInput__CFlightStick** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IFlightStick** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFlightStickStatics = __uuidof(IFlightStickStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGameController[] = L"Windows.Gaming.Input.IGameController";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("1baf6522-5f64-42c5-8267-b9fe2215bfbd")
                IGameController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_HeadsetConnected(
                        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HeadsetConnected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HeadsetDisconnected(
                        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HeadsetDisconnected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UserChanged(
                        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UserChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Headset(
                        ABI::Windows::Gaming::Input::IHeadset** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWireless(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameController = __uuidof(IGameController);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGameController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGameControllerBatteryInfo[] = L"Windows.Gaming.Input.IGameControllerBatteryInfo";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("dcecc681-3963-4da6-955d-553f3b6f6161")
                IGameControllerBatteryInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryGetBatteryReport(
                        ABI::Windows::Devices::Power::IBatteryReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameControllerBatteryInfo = __uuidof(IGameControllerBatteryInfo);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IGamepad
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepad[] = L"Windows.Gaming.Input.IGamepad";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("bc7bb43c-0a69-3903-9e9d-a50f86a45de5")
                IGamepad : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Vibration(
                        ABI::Windows::Gaming::Input::GamepadVibration* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Vibration(
                        ABI::Windows::Gaming::Input::GamepadVibration value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        ABI::Windows::Gaming::Input::GamepadReading* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGamepad = __uuidof(IGamepad);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepad;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGamepad2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGamepad
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepad2[] = L"Windows.Gaming.Input.IGamepad2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("3c1689bd-5915-4245-b0c0-c89fae0308ff")
                IGamepad2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetButtonLabel(
                        ABI::Windows::Gaming::Input::GamepadButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGamepad2 = __uuidof(IGamepad2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepad2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IGamepadStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepadStatics[] = L"Windows.Gaming.Input.IGamepadStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("8bbce529-d49c-39e9-9560-e47dde96b7c8")
                IGamepadStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_GamepadAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CGamepad* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_GamepadAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_GamepadRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CGamepad* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_GamepadRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gamepads(
                        __FIVectorView_1_Windows__CGaming__CInput__CGamepad** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGamepadStatics = __uuidof(IGamepadStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepadStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGamepadStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGamepadStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepadStatics2[] = L"Windows.Gaming.Input.IGamepadStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("42676dc5-0856-47c4-9213-b395504c3a3c")
                IGamepadStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IGamepad** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGamepadStatics2 = __uuidof(IGamepadStatics2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IHeadset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Headset
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IHeadset[] = L"Windows.Gaming.Input.IHeadset";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("3fd156ef-6925-3fa8-9181-029c5223ae3b")
                IHeadset : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CaptureDeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RenderDeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHeadset = __uuidof(IHeadset);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIHeadset;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheel[] = L"Windows.Gaming.Input.IRacingWheel";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("f546656f-e106-4c82-a90f-554012904b85")
                IRacingWheel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasClutch(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasHandbrake(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasPatternShifter(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPatternShifterGear(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxWheelAngle(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WheelMotor(
                        ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetButtonLabel(
                        ABI::Windows::Gaming::Input::RacingWheelButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        ABI::Windows::Gaming::Input::RacingWheelReading* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRacingWheel = __uuidof(IRacingWheel);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheel;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheelStatics[] = L"Windows.Gaming.Input.IRacingWheelStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("3ac12cd5-581b-4936-9f94-69f1e6514c7d")
                IRacingWheelStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_RacingWheelAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RacingWheelAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RacingWheelRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RacingWheelRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RacingWheels(
                        __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRacingWheelStatics = __uuidof(IRacingWheelStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheelStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IRacingWheelStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheelStatics2[] = L"Windows.Gaming.Input.IRacingWheelStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("e666bcaa-edfd-4323-a9f6-3c384048d1ed")
                IRacingWheelStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IRacingWheel** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRacingWheelStatics2 = __uuidof(IRacingWheelStatics2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameController[] = L"Windows.Gaming.Input.IRawGameController";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("7cad6d91-a7e1-4f71-9a78-33e9c5dfea62")
                IRawGameController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AxisCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForceFeedbackMotors(
                        __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareProductId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareVendorId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SwitchCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetButtonLabel(
                        INT32 buttonIndex,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        UINT32 buttonArrayLength,
                        boolean* buttonArray,
                        UINT32 switchArrayLength,
                        ABI::Windows::Gaming::Input::GameControllerSwitchPosition* switchArray,
                        UINT32 axisArrayLength,
                        DOUBLE* axisArray,
                        UINT64* timestamp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSwitchKind(
                        INT32 switchIndex,
                        ABI::Windows::Gaming::Input::GameControllerSwitchKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawGameController = __uuidof(IRawGameController);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IRawGameController
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameController2[] = L"Windows.Gaming.Input.IRawGameController2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("43c0c035-bb73-4756-a787-3ed6bea617bd")
                IRawGameController2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsControllers(
                        __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NonRoamableId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawGameController2 = __uuidof(IRawGameController2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameController2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameControllerStatics[] = L"Windows.Gaming.Input.IRawGameControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("eb8d0792-e95a-4b19-afc7-0a59f8bf759e")
                IRawGameControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_RawGameControllerAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RawGameControllerAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RawGameControllerRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RawGameControllerRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawGameControllers(
                        __FIVectorView_1_Windows__CGaming__CInput__CRawGameController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IRawGameController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRawGameControllerStatics = __uuidof(IRawGameControllerStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationController[] = L"Windows.Gaming.Input.IUINavigationController";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("e5aeefdd-f50e-4a55-8cdc-d33229548175")
                IUINavigationController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                        ABI::Windows::Gaming::Input::UINavigationReading* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOptionalButtonLabel(
                        ABI::Windows::Gaming::Input::OptionalUINavigationButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRequiredButtonLabel(
                        ABI::Windows::Gaming::Input::RequiredUINavigationButtons button,
                        ABI::Windows::Gaming::Input::GameControllerButtonLabel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUINavigationController = __uuidof(IUINavigationController);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationControllerStatics[] = L"Windows.Gaming.Input.IUINavigationControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("2f14930a-f6f8-4a48-8d89-94786cca0c2e")
                IUINavigationControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_UINavigationControllerAdded(
                        __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UINavigationControllerAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UINavigationControllerRemoved(
                        __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UINavigationControllerRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UINavigationControllers(
                        __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUINavigationControllerStatics = __uuidof(IUINavigationControllerStatics);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationControllerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IUINavigationControllerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationControllerStatics2[] = L"Windows.Gaming.Input.IUINavigationControllerStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                MIDL_INTERFACE("e0cb28e3-b20b-4b0b-9ed4-f3d53cec0de4")
                IUINavigationControllerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromGameController(
                        ABI::Windows::Gaming::Input::IGameController* gameController,
                        ABI::Windows::Gaming::Input::IUINavigationController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUINavigationControllerStatics2 = __uuidof(IUINavigationControllerStatics2);
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.ArcadeStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IArcadeStickStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IArcadeStickStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IArcadeStick ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ArcadeStick_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ArcadeStick_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ArcadeStick[] = L"Windows.Gaming.Input.ArcadeStick";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.FlightStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IFlightStickStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IFlightStick ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_FlightStick_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_FlightStick_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_FlightStick[] = L"Windows.Gaming.Input.FlightStick";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.Gamepad
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IGamepadStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IGamepadStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IGamepad ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGamepad2
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Gamepad_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Gamepad_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Gamepad[] = L"Windows.Gaming.Input.Gamepad";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.Headset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IHeadset ** Default Interface **
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Headset_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Headset_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Headset[] = L"Windows.Gaming.Input.Headset";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.RacingWheel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IRacingWheelStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IRacingWheelStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IRacingWheel ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_RacingWheel_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_RacingWheel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_RacingWheel[] = L"Windows.Gaming.Input.RacingWheel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.RawGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IRawGameControllerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IRawGameController ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *    Windows.Gaming.Input.IRawGameController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_RawGameController_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_RawGameController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_RawGameController[] = L"Windows.Gaming.Input.RawGameController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.UINavigationController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IUINavigationControllerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IUINavigationControllerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IUINavigationController ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_UINavigationController_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_UINavigationController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_UINavigationController[] = L"Windows.Gaming.Input.UINavigationController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStick __x_ABI_CWindows_CGaming_CInput_CIArcadeStick;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2 __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIFlightStick __x_ABI_CWindows_CGaming_CInput_CIFlightStick;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGameController __x_ABI_CWindows_CGaming_CInput_CIGameController;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGamepad __x_ABI_CWindows_CGaming_CInput_CIGamepad;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepad_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGamepad2 __x_ABI_CWindows_CGaming_CInput_CIGamepad2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2 __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIHeadset __x_ABI_CWindows_CGaming_CInput_CIHeadset;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIHeadset_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheel __x_ABI_CWindows_CGaming_CInput_CIRacingWheel;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2 __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRawGameController __x_ABI_CWindows_CGaming_CInput_CIRawGameController;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRawGameController2 __x_ABI_CWindows_CGaming_CInput_CIRawGameController2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationController __x_ABI_CWindows_CGaming_CInput_CIUINavigationController;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2 __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

typedef struct __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl;

interface __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

typedef struct __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl;

interface __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CArcadeStick __FIIterator_1_Windows__CGaming__CInput__CArcadeStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CArcadeStick;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CArcadeStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CArcadeStick* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CArcadeStickVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CArcadeStick
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CArcadeStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CArcadeStick_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CArcadeStick __FIIterable_1_Windows__CGaming__CInput__CArcadeStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CArcadeStick;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CArcadeStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CArcadeStick* This,
        __FIIterator_1_Windows__CGaming__CInput__CArcadeStick** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CArcadeStickVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CArcadeStick
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CArcadeStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CArcadeStick_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CFlightStick __FIIterator_1_Windows__CGaming__CInput__CFlightStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CFlightStick;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CFlightStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CFlightStick* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CFlightStickVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CFlightStick
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CFlightStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CFlightStick_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CFlightStick __FIIterable_1_Windows__CGaming__CInput__CFlightStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CFlightStick;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CFlightStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CFlightStick* This,
        __FIIterator_1_Windows__CGaming__CInput__CFlightStick** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CFlightStickVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CFlightStick
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CFlightStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CFlightStick_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        __FIIterator_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CGamepad __FIIterator_1_Windows__CGaming__CInput__CGamepad;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CGamepad;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CGamepadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CGamepad* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CGamepadVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CGamepad
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CGamepadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CGamepad_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CGamepad __FIIterable_1_Windows__CGaming__CInput__CGamepad;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CGamepad;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CGamepadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CGamepad* This,
        __FIIterator_1_Windows__CGaming__CInput__CGamepad** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CGamepadVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CGamepad
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CGamepadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CGamepad_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CRacingWheel __FIIterator_1_Windows__CGaming__CInput__CRacingWheel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CRacingWheel;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CRacingWheelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CRacingWheel* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CRacingWheelVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CRacingWheel
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CRacingWheelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRacingWheel_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CRacingWheel __FIIterable_1_Windows__CGaming__CInput__CRacingWheel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CRacingWheel;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CRacingWheelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CRacingWheel* This,
        __FIIterator_1_Windows__CGaming__CInput__CRacingWheel** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CRacingWheelVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CRacingWheel
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CRacingWheelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CRacingWheel_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CRawGameController __FIIterator_1_Windows__CGaming__CInput__CRawGameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CRawGameController;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CRawGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CRawGameController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CRawGameControllerVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CRawGameController
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CRawGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CRawGameController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CRawGameController __FIIterable_1_Windows__CGaming__CInput__CRawGameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CRawGameController;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CRawGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CRawGameController* This,
        __FIIterator_1_Windows__CGaming__CInput__CRawGameController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CRawGameControllerVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CRawGameController
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CRawGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CRawGameController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CInput__CUINavigationController __FIIterator_1_Windows__CGaming__CInput__CUINavigationController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CInput__CUINavigationController;

typedef struct __FIIterator_1_Windows__CGaming__CInput__CUINavigationControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CInput__CUINavigationController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CInput__CUINavigationControllerVtbl;

interface __FIIterator_1_Windows__CGaming__CInput__CUINavigationController
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CInput__CUINavigationControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CInput__CUINavigationController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CInput__CUINavigationController __FIIterable_1_Windows__CGaming__CInput__CUINavigationController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CInput__CUINavigationController;

typedef struct __FIIterable_1_Windows__CGaming__CInput__CUINavigationControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CInput__CUINavigationController* This,
        __FIIterator_1_Windows__CGaming__CInput__CUINavigationController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CInput__CUINavigationControllerVtbl;

interface __FIIterable_1_Windows__CGaming__CInput__CUINavigationController
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CInput__CUINavigationControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CInput__CUINavigationController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController;

typedef struct __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl;

interface __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CArcadeStick;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CArcadeStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CArcadeStick* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CArcadeStickVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CArcadeStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CFlightStick __FIVectorView_1_Windows__CGaming__CInput__CFlightStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CFlightStick;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CFlightStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CFlightStick* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CFlightStickVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CFlightStick
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CFlightStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CFlightStick_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CGamepad __FIVectorView_1_Windows__CGaming__CInput__CGamepad;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CGamepad;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CGamepadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CGamepad* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CGamepadVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CGamepad
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CGamepadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CGamepad_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CRacingWheel;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CRacingWheelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CRacingWheel* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CRacingWheelVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CRacingWheelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CRawGameController __FIVectorView_1_Windows__CGaming__CInput__CRawGameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CRawGameController;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CRawGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CRawGameController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CRawGameControllerVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CRawGameController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CRawGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CRawGameController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CInput__CUINavigationController;

typedef struct __FIVectorView_1_Windows__CGaming__CInput__CUINavigationControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CInput__CUINavigationController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CInput__CUINavigationControllerVtbl;

interface __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CInput__CUINavigationControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStickVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CFlightStick;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CFlightStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CFlightStickVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CFlightStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CFlightStick_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CGamepad __FIEventHandler_1_Windows__CGaming__CInput__CGamepad;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CGamepad;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CGamepadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CGamepad* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CGamepad* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CGamepad* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CGamepad* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CGamepadVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CGamepad
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CGamepadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CGamepad_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CGamepad_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CGamepad_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CGamepad_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CGamepad_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheelVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CRawGameController;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CRawGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CRawGameControllerVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CRawGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CRawGameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController;

typedef struct __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* This,
        IInspectable* sender,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationControllerVtbl;

interface __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset;

typedef struct __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadsetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* sender,
        __x_ABI_CWindows_CGaming_CInput_CIHeadset* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadsetVtbl;

interface __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadsetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUserChangedEventArgs __x_ABI_CWindows_CSystem_CIUserChangedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CIUserChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* sender,
        __x_ABI_CWindows_CSystem_CIUserChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIBatteryReport __x_ABI_CWindows_CDevices_CPower_CIBatteryReport;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CInput_CArcadeStickButtons __x_ABI_CWindows_CGaming_CInput_CArcadeStickButtons;

typedef enum __x_ABI_CWindows_CGaming_CInput_CFlightStickButtons __x_ABI_CWindows_CGaming_CInput_CFlightStickButtons;

typedef enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel;

typedef enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchKind __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchKind;

typedef enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchPosition __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchPosition;

typedef enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons __x_ABI_CWindows_CGaming_CInput_CGamepadButtons;

typedef enum __x_ABI_CWindows_CGaming_CInput_COptionalUINavigationButtons __x_ABI_CWindows_CGaming_CInput_COptionalUINavigationButtons;

typedef enum __x_ABI_CWindows_CGaming_CInput_CRacingWheelButtons __x_ABI_CWindows_CGaming_CInput_CRacingWheelButtons;

typedef enum __x_ABI_CWindows_CGaming_CInput_CRequiredUINavigationButtons __x_ABI_CWindows_CGaming_CInput_CRequiredUINavigationButtons;

typedef struct __x_ABI_CWindows_CGaming_CInput_CArcadeStickReading __x_ABI_CWindows_CGaming_CInput_CArcadeStickReading;

typedef struct __x_ABI_CWindows_CGaming_CInput_CFlightStickReading __x_ABI_CWindows_CGaming_CInput_CFlightStickReading;

typedef struct __x_ABI_CWindows_CGaming_CInput_CGamepadReading __x_ABI_CWindows_CGaming_CInput_CGamepadReading;

typedef struct __x_ABI_CWindows_CGaming_CInput_CGamepadVibration __x_ABI_CWindows_CGaming_CInput_CGamepadVibration;

typedef struct __x_ABI_CWindows_CGaming_CInput_CRacingWheelReading __x_ABI_CWindows_CGaming_CInput_CRacingWheelReading;

typedef struct __x_ABI_CWindows_CGaming_CInput_CUINavigationReading __x_ABI_CWindows_CGaming_CInput_CUINavigationReading;

/*
 *
 * Struct Windows.Gaming.Input.ArcadeStickButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CArcadeStickButtons
{
    ArcadeStickButtons_None = 0,
    ArcadeStickButtons_StickUp = 0x1,
    ArcadeStickButtons_StickDown = 0x2,
    ArcadeStickButtons_StickLeft = 0x4,
    ArcadeStickButtons_StickRight = 0x8,
    ArcadeStickButtons_Action1 = 0x10,
    ArcadeStickButtons_Action2 = 0x20,
    ArcadeStickButtons_Action3 = 0x40,
    ArcadeStickButtons_Action4 = 0x80,
    ArcadeStickButtons_Action5 = 0x100,
    ArcadeStickButtons_Action6 = 0x200,
    ArcadeStickButtons_Special1 = 0x400,
    ArcadeStickButtons_Special2 = 0x800,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.FlightStickButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGaming_CInput_CFlightStickButtons
{
    FlightStickButtons_None = 0,
    FlightStickButtons_FirePrimary = 0x1,
    FlightStickButtons_FireSecondary = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerButtonLabel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel
{
    GameControllerButtonLabel_None = 0,
    GameControllerButtonLabel_XboxBack = 1,
    GameControllerButtonLabel_XboxStart = 2,
    GameControllerButtonLabel_XboxMenu = 3,
    GameControllerButtonLabel_XboxView = 4,
    GameControllerButtonLabel_XboxUp = 5,
    GameControllerButtonLabel_XboxDown = 6,
    GameControllerButtonLabel_XboxLeft = 7,
    GameControllerButtonLabel_XboxRight = 8,
    GameControllerButtonLabel_XboxA = 9,
    GameControllerButtonLabel_XboxB = 10,
    GameControllerButtonLabel_XboxX = 11,
    GameControllerButtonLabel_XboxY = 12,
    GameControllerButtonLabel_XboxLeftBumper = 13,
    GameControllerButtonLabel_XboxLeftTrigger = 14,
    GameControllerButtonLabel_XboxLeftStickButton = 15,
    GameControllerButtonLabel_XboxRightBumper = 16,
    GameControllerButtonLabel_XboxRightTrigger = 17,
    GameControllerButtonLabel_XboxRightStickButton = 18,
    GameControllerButtonLabel_XboxPaddle1 = 19,
    GameControllerButtonLabel_XboxPaddle2 = 20,
    GameControllerButtonLabel_XboxPaddle3 = 21,
    GameControllerButtonLabel_XboxPaddle4 = 22,
    GameControllerButtonLabel_Mode = 23,
    GameControllerButtonLabel_Select = 24,
    GameControllerButtonLabel_Menu = 25,
    GameControllerButtonLabel_View = 26,
    GameControllerButtonLabel_Back = 27,
    GameControllerButtonLabel_Start = 28,
    GameControllerButtonLabel_Options = 29,
    GameControllerButtonLabel_Share = 30,
    GameControllerButtonLabel_Up = 31,
    GameControllerButtonLabel_Down = 32,
    GameControllerButtonLabel_Left = 33,
    GameControllerButtonLabel_Right = 34,
    GameControllerButtonLabel_LetterA = 35,
    GameControllerButtonLabel_LetterB = 36,
    GameControllerButtonLabel_LetterC = 37,
    GameControllerButtonLabel_LetterL = 38,
    GameControllerButtonLabel_LetterR = 39,
    GameControllerButtonLabel_LetterX = 40,
    GameControllerButtonLabel_LetterY = 41,
    GameControllerButtonLabel_LetterZ = 42,
    GameControllerButtonLabel_Cross = 43,
    GameControllerButtonLabel_Circle = 44,
    GameControllerButtonLabel_Square = 45,
    GameControllerButtonLabel_Triangle = 46,
    GameControllerButtonLabel_LeftBumper = 47,
    GameControllerButtonLabel_LeftTrigger = 48,
    GameControllerButtonLabel_LeftStickButton = 49,
    GameControllerButtonLabel_Left1 = 50,
    GameControllerButtonLabel_Left2 = 51,
    GameControllerButtonLabel_Left3 = 52,
    GameControllerButtonLabel_RightBumper = 53,
    GameControllerButtonLabel_RightTrigger = 54,
    GameControllerButtonLabel_RightStickButton = 55,
    GameControllerButtonLabel_Right1 = 56,
    GameControllerButtonLabel_Right2 = 57,
    GameControllerButtonLabel_Right3 = 58,
    GameControllerButtonLabel_Paddle1 = 59,
    GameControllerButtonLabel_Paddle2 = 60,
    GameControllerButtonLabel_Paddle3 = 61,
    GameControllerButtonLabel_Paddle4 = 62,
    GameControllerButtonLabel_Plus = 63,
    GameControllerButtonLabel_Minus = 64,
    GameControllerButtonLabel_DownLeftArrow = 65,
    GameControllerButtonLabel_DialLeft = 66,
    GameControllerButtonLabel_DialRight = 67,
    GameControllerButtonLabel_Suspension = 68,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerSwitchKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchKind
{
    GameControllerSwitchKind_TwoWay = 0,
    GameControllerSwitchKind_FourWay = 1,
    GameControllerSwitchKind_EightWay = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GameControllerSwitchPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchPosition
{
    GameControllerSwitchPosition_Center = 0,
    GameControllerSwitchPosition_Up = 1,
    GameControllerSwitchPosition_UpRight = 2,
    GameControllerSwitchPosition_Right = 3,
    GameControllerSwitchPosition_DownRight = 4,
    GameControllerSwitchPosition_Down = 5,
    GameControllerSwitchPosition_DownLeft = 6,
    GameControllerSwitchPosition_Left = 7,
    GameControllerSwitchPosition_UpLeft = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GamepadButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons
{
    GamepadButtons_None = 0,
    GamepadButtons_Menu = 0x1,
    GamepadButtons_View = 0x2,
    GamepadButtons_A = 0x4,
    GamepadButtons_B = 0x8,
    GamepadButtons_X = 0x10,
    GamepadButtons_Y = 0x20,
    GamepadButtons_DPadUp = 0x40,
    GamepadButtons_DPadDown = 0x80,
    GamepadButtons_DPadLeft = 0x100,
    GamepadButtons_DPadRight = 0x200,
    GamepadButtons_LeftShoulder = 0x400,
    GamepadButtons_RightShoulder = 0x800,
    GamepadButtons_LeftThumbstick = 0x1000,
    GamepadButtons_RightThumbstick = 0x2000,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    GamepadButtons_Paddle1 = 0x4000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    GamepadButtons_Paddle2 = 0x8000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    GamepadButtons_Paddle3 = 0x10000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    GamepadButtons_Paddle4 = 0x20000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.OptionalUINavigationButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_COptionalUINavigationButtons
{
    OptionalUINavigationButtons_None = 0,
    OptionalUINavigationButtons_Context1 = 0x1,
    OptionalUINavigationButtons_Context2 = 0x2,
    OptionalUINavigationButtons_Context3 = 0x4,
    OptionalUINavigationButtons_Context4 = 0x8,
    OptionalUINavigationButtons_PageUp = 0x10,
    OptionalUINavigationButtons_PageDown = 0x20,
    OptionalUINavigationButtons_PageLeft = 0x40,
    OptionalUINavigationButtons_PageRight = 0x80,
    OptionalUINavigationButtons_ScrollUp = 0x100,
    OptionalUINavigationButtons_ScrollDown = 0x200,
    OptionalUINavigationButtons_ScrollLeft = 0x400,
    OptionalUINavigationButtons_ScrollRight = 0x800,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.RacingWheelButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CRacingWheelButtons
{
    RacingWheelButtons_None = 0,
    RacingWheelButtons_PreviousGear = 0x1,
    RacingWheelButtons_NextGear = 0x2,
    RacingWheelButtons_DPadUp = 0x4,
    RacingWheelButtons_DPadDown = 0x8,
    RacingWheelButtons_DPadLeft = 0x10,
    RacingWheelButtons_DPadRight = 0x20,
    RacingWheelButtons_Button1 = 0x40,
    RacingWheelButtons_Button2 = 0x80,
    RacingWheelButtons_Button3 = 0x100,
    RacingWheelButtons_Button4 = 0x200,
    RacingWheelButtons_Button5 = 0x400,
    RacingWheelButtons_Button6 = 0x800,
    RacingWheelButtons_Button7 = 0x1000,
    RacingWheelButtons_Button8 = 0x2000,
    RacingWheelButtons_Button9 = 0x4000,
    RacingWheelButtons_Button10 = 0x8000,
    RacingWheelButtons_Button11 = 0x10000,
    RacingWheelButtons_Button12 = 0x20000,
    RacingWheelButtons_Button13 = 0x40000,
    RacingWheelButtons_Button14 = 0x80000,
    RacingWheelButtons_Button15 = 0x100000,
    RacingWheelButtons_Button16 = 0x200000,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.RequiredUINavigationButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CRequiredUINavigationButtons
{
    RequiredUINavigationButtons_None = 0,
    RequiredUINavigationButtons_Menu = 0x1,
    RequiredUINavigationButtons_View = 0x2,
    RequiredUINavigationButtons_Accept = 0x4,
    RequiredUINavigationButtons_Cancel = 0x8,
    RequiredUINavigationButtons_Up = 0x10,
    RequiredUINavigationButtons_Down = 0x20,
    RequiredUINavigationButtons_Left = 0x40,
    RequiredUINavigationButtons_Right = 0x80,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ArcadeStickReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CGaming_CInput_CArcadeStickReading
{
    UINT64 Timestamp;
    enum __x_ABI_CWindows_CGaming_CInput_CArcadeStickButtons Buttons;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.FlightStickReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
struct __x_ABI_CWindows_CGaming_CInput_CFlightStickReading
{
    UINT64 Timestamp;
    enum __x_ABI_CWindows_CGaming_CInput_CFlightStickButtons Buttons;
    enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchPosition HatSwitch;
    DOUBLE Roll;
    DOUBLE Pitch;
    DOUBLE Yaw;
    DOUBLE Throttle;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.Input.GamepadReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGaming_CInput_CGamepadReading
{
    UINT64 Timestamp;
    enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons Buttons;
    DOUBLE LeftTrigger;
    DOUBLE RightTrigger;
    DOUBLE LeftThumbstickX;
    DOUBLE LeftThumbstickY;
    DOUBLE RightThumbstickX;
    DOUBLE RightThumbstickY;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.GamepadVibration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGaming_CInput_CGamepadVibration
{
    DOUBLE LeftMotor;
    DOUBLE RightMotor;
    DOUBLE LeftTrigger;
    DOUBLE RightTrigger;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Input.RacingWheelReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CGaming_CInput_CRacingWheelReading
{
    UINT64 Timestamp;
    enum __x_ABI_CWindows_CGaming_CInput_CRacingWheelButtons Buttons;
    INT32 PatternShifterGear;
    DOUBLE Wheel;
    DOUBLE Throttle;
    DOUBLE Brake;
    DOUBLE Clutch;
    DOUBLE Handbrake;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.UINavigationReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CGaming_CInput_CUINavigationReading
{
    UINT64 Timestamp;
    enum __x_ABI_CWindows_CGaming_CInput_CRequiredUINavigationButtons RequiredButtons;
    enum __x_ABI_CWindows_CGaming_CInput_COptionalUINavigationButtons OptionalButtons;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStick[] = L"Windows.Gaming.Input.IArcadeStick";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        enum __x_ABI_CWindows_CGaming_CInput_CArcadeStickButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStick* This,
        struct __x_ABI_CWindows_CGaming_CInput_CArcadeStickReading* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIArcadeStickVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStick
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_GetButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetButtonLabel(This, button, value))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStick_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStick;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStick_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStickStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStickStatics[] = L"Windows.Gaming.Input.IArcadeStickStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ArcadeStickAdded)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ArcadeStickAdded)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ArcadeStickRemoved)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CArcadeStick* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ArcadeStickRemoved)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_ArcadeSticks)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CArcadeStick** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_add_ArcadeStickAdded(This, value, token) \
    ((This)->lpVtbl->add_ArcadeStickAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_remove_ArcadeStickAdded(This, token) \
    ((This)->lpVtbl->remove_ArcadeStickAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_add_ArcadeStickRemoved(This, value, token) \
    ((This)->lpVtbl->add_ArcadeStickRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_remove_ArcadeStickRemoved(This, token) \
    ((This)->lpVtbl->remove_ArcadeStickRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_get_ArcadeSticks(This, value) \
    ((This)->lpVtbl->get_ArcadeSticks(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IArcadeStickStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ArcadeStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IArcadeStickStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IArcadeStickStatics2[] = L"Windows.Gaming.Input.IArcadeStickStatics2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIArcadeStick** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIArcadeStickStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IFlightStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.FlightStick
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IFlightStick[] = L"Windows.Gaming.Input.IFlightStick";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIFlightStickVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HatSwitchKind)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchKind* value);
    HRESULT (STDMETHODCALLTYPE* GetButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        enum __x_ABI_CWindows_CGaming_CInput_CFlightStickButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIFlightStick* This,
        struct __x_ABI_CWindows_CGaming_CInput_CFlightStickReading* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIFlightStickVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIFlightStick
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIFlightStickVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_get_HatSwitchKind(This, value) \
    ((This)->lpVtbl->get_HatSwitchKind(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_GetButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetButtonLabel(This, button, value))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStick_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIFlightStick;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStick_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IFlightStickStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.FlightStick
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IFlightStickStatics[] = L"Windows.Gaming.Input.IFlightStickStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIFlightStickStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_FlightStickAdded)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FlightStickAdded)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FlightStickRemoved)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CFlightStick* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FlightStickRemoved)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_FlightSticks)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CFlightStick** value);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIFlightStick** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIFlightStickStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIFlightStickStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_add_FlightStickAdded(This, value, token) \
    ((This)->lpVtbl->add_FlightStickAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_remove_FlightStickAdded(This, token) \
    ((This)->lpVtbl->remove_FlightStickAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_add_FlightStickRemoved(This, value, token) \
    ((This)->lpVtbl->add_FlightStickRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_remove_FlightStickRemoved(This, token) \
    ((This)->lpVtbl->remove_FlightStickRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_get_FlightSticks(This, value) \
    ((This)->lpVtbl->get_FlightSticks(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIFlightStickStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGameController[] = L"Windows.Gaming.Input.IGameController";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_HeadsetConnected)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HeadsetConnected)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HeadsetDisconnected)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CGaming__CInput__CHeadset* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HeadsetDisconnected)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UserChanged)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        __FITypedEventHandler_2_Windows__CGaming__CInput__CIGameController_Windows__CSystem__CUserChangedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UserChanged)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Headset)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        __x_ABI_CWindows_CGaming_CInput_CIHeadset** value);
    HRESULT (STDMETHODCALLTYPE* get_IsWireless)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CGaming_CInput_CIGameController* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGameControllerVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGameController
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_add_HeadsetConnected(This, value, token) \
    ((This)->lpVtbl->add_HeadsetConnected(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_remove_HeadsetConnected(This, token) \
    ((This)->lpVtbl->remove_HeadsetConnected(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_add_HeadsetDisconnected(This, value, token) \
    ((This)->lpVtbl->add_HeadsetDisconnected(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_remove_HeadsetDisconnected(This, token) \
    ((This)->lpVtbl->remove_HeadsetDisconnected(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_add_UserChanged(This, value, token) \
    ((This)->lpVtbl->add_UserChanged(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_remove_UserChanged(This, token) \
    ((This)->lpVtbl->remove_UserChanged(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_get_Headset(This, value) \
    ((This)->lpVtbl->get_Headset(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_get_IsWireless(This, value) \
    ((This)->lpVtbl->get_IsWireless(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIGameController_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGameController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGameControllerBatteryInfo[] = L"Windows.Gaming.Input.IGameControllerBatteryInfo";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetBatteryReport)(__x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo* This,
        __x_ABI_CWindows_CDevices_CPower_CIBatteryReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfoVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_TryGetBatteryReport(This, value) \
    ((This)->lpVtbl->TryGetBatteryReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGameControllerBatteryInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IGamepad
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepad[] = L"Windows.Gaming.Input.IGamepad";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGamepadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Vibration)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        struct __x_ABI_CWindows_CGaming_CInput_CGamepadVibration* value);
    HRESULT (STDMETHODCALLTYPE* put_Vibration)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        struct __x_ABI_CWindows_CGaming_CInput_CGamepadVibration value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIGamepad* This,
        struct __x_ABI_CWindows_CGaming_CInput_CGamepadReading* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGamepadVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGamepad
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGamepadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_get_Vibration(This, value) \
    ((This)->lpVtbl->get_Vibration(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_put_Vibration(This, value) \
    ((This)->lpVtbl->put_Vibration(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepad;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGamepad2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGamepad
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepad2[] = L"Windows.Gaming.Input.IGamepad2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGamepad2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIGamepad2* This,
        enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGamepad2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGamepad2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGamepad2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepad2_GetButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetButtonLabel(This, button, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepad2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepad2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IGamepadStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepadStatics[] = L"Windows.Gaming.Input.IGamepadStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGamepadStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_GamepadAdded)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CGamepad* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GamepadAdded)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GamepadRemoved)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CGamepad* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GamepadRemoved)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Gamepads)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CGamepad** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGamepadStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGamepadStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_add_GamepadAdded(This, value, token) \
    ((This)->lpVtbl->add_GamepadAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_remove_GamepadAdded(This, token) \
    ((This)->lpVtbl->remove_GamepadAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_add_GamepadRemoved(This, value, token) \
    ((This)->lpVtbl->add_GamepadRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_remove_GamepadRemoved(This, token) \
    ((This)->lpVtbl->remove_GamepadRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_get_Gamepads(This, value) \
    ((This)->lpVtbl->get_Gamepads(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepadStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IGamepadStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Gamepad
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGamepadStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IGamepadStatics2[] = L"Windows.Gaming.Input.IGamepadStatics2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIGamepad** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIGamepadStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IHeadset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Headset
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IHeadset[] = L"Windows.Gaming.Input.IHeadset";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIHeadsetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CaptureDeviceId)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RenderDeviceId)(__x_ABI_CWindows_CGaming_CInput_CIHeadset* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIHeadsetVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIHeadset
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIHeadsetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_get_CaptureDeviceId(This, value) \
    ((This)->lpVtbl->get_CaptureDeviceId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIHeadset_get_RenderDeviceId(This, value) \
    ((This)->lpVtbl->get_RenderDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIHeadset;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIHeadset_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheel[] = L"Windows.Gaming.Input.IRacingWheel";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasClutch)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasHandbrake)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasPatternShifter)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPatternShifterGear)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxWheelAngle)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_WheelMotor)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor** value);
    HRESULT (STDMETHODCALLTYPE* GetButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        enum __x_ABI_CWindows_CGaming_CInput_CRacingWheelButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheel* This,
        struct __x_ABI_CWindows_CGaming_CInput_CRacingWheelReading* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRacingWheelVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheel
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_HasClutch(This, value) \
    ((This)->lpVtbl->get_HasClutch(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_HasHandbrake(This, value) \
    ((This)->lpVtbl->get_HasHandbrake(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_HasPatternShifter(This, value) \
    ((This)->lpVtbl->get_HasPatternShifter(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_MaxPatternShifterGear(This, value) \
    ((This)->lpVtbl->get_MaxPatternShifterGear(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_MaxWheelAngle(This, value) \
    ((This)->lpVtbl->get_MaxWheelAngle(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_get_WheelMotor(This, value) \
    ((This)->lpVtbl->get_WheelMotor(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_GetButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetButtonLabel(This, button, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheel_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheel;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheelStatics[] = L"Windows.Gaming.Input.IRacingWheelStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_RacingWheelAdded)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RacingWheelAdded)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RacingWheelRemoved)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CRacingWheel* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RacingWheelRemoved)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_RacingWheels)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CRacingWheel** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_add_RacingWheelAdded(This, value, token) \
    ((This)->lpVtbl->add_RacingWheelAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_remove_RacingWheelAdded(This, token) \
    ((This)->lpVtbl->remove_RacingWheelAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_add_RacingWheelRemoved(This, value, token) \
    ((This)->lpVtbl->add_RacingWheelRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_remove_RacingWheelRemoved(This, token) \
    ((This)->lpVtbl->remove_RacingWheelRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_get_RacingWheels(This, value) \
    ((This)->lpVtbl->get_RacingWheels(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IRacingWheelStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RacingWheel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IRacingWheelStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRacingWheelStatics2[] = L"Windows.Gaming.Input.IRacingWheelStatics2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIRacingWheel** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRacingWheelStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameController[] = L"Windows.Gaming.Input.IRawGameController";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AxisCount)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonCount)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ForceFeedbackMotors)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        __FIVectorView_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackMotor** value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareProductId)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareVendorId)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_SwitchCount)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        INT32 buttonIndex,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        UINT32 buttonArrayLength,
        boolean* buttonArray,
        UINT32 switchArrayLength,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchPosition* switchArray,
        UINT32 axisArrayLength,
        DOUBLE* axisArray,
        UINT64* timestamp);
    HRESULT (STDMETHODCALLTYPE* GetSwitchKind)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController* This,
        INT32 switchIndex,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerSwitchKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRawGameController
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_AxisCount(This, value) \
    ((This)->lpVtbl->get_AxisCount(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_ButtonCount(This, value) \
    ((This)->lpVtbl->get_ButtonCount(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_ForceFeedbackMotors(This, value) \
    ((This)->lpVtbl->get_ForceFeedbackMotors(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_HardwareProductId(This, value) \
    ((This)->lpVtbl->get_HardwareProductId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_HardwareVendorId(This, value) \
    ((This)->lpVtbl->get_HardwareVendorId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_get_SwitchCount(This, value) \
    ((This)->lpVtbl->get_SwitchCount(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetButtonLabel(This, buttonIndex, value) \
    ((This)->lpVtbl->GetButtonLabel(This, buttonIndex, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetCurrentReading(This, buttonArrayLength, buttonArray, switchArrayLength, switchArray, axisArrayLength, axisArray, timestamp) \
    ((This)->lpVtbl->GetCurrentReading(This, buttonArrayLength, buttonArray, switchArrayLength, switchArray, axisArrayLength, axisArray, timestamp))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController_GetSwitchKind(This, switchIndex, value) \
    ((This)->lpVtbl->GetSwitchKind(This, switchIndex, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IRawGameController
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameController2[] = L"Windows.Gaming.Input.IRawGameController2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRawGameController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsControllers)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsController** value);
    HRESULT (STDMETHODCALLTYPE* get_NonRoamableId)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGaming_CInput_CIRawGameController2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRawGameController2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRawGameController2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRawGameController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_get_SimpleHapticsControllers(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsControllers(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_get_NonRoamableId(This, value) \
    ((This)->lpVtbl->get_NonRoamableId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameController2_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameController2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Gaming.Input.IRawGameControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.RawGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IRawGameControllerStatics[] = L"Windows.Gaming.Input.IRawGameControllerStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_RawGameControllerAdded)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RawGameControllerAdded)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RawGameControllerRemoved)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CRawGameController* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RawGameControllerRemoved)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_RawGameControllers)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CRawGameController** value);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIRawGameController** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_add_RawGameControllerAdded(This, value, token) \
    ((This)->lpVtbl->add_RawGameControllerAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_remove_RawGameControllerAdded(This, token) \
    ((This)->lpVtbl->remove_RawGameControllerAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_add_RawGameControllerRemoved(This, value, token) \
    ((This)->lpVtbl->add_RawGameControllerRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_remove_RawGameControllerRemoved(This, token) \
    ((This)->lpVtbl->remove_RawGameControllerRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_get_RawGameControllers(This, value) \
    ((This)->lpVtbl->get_RawGameControllers(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIRawGameControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IGameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationController[] = L"Windows.Gaming.Input.IUINavigationController";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        struct __x_ABI_CWindows_CGaming_CInput_CUINavigationReading* value);
    HRESULT (STDMETHODCALLTYPE* GetOptionalButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        enum __x_ABI_CWindows_CGaming_CInput_COptionalUINavigationButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);
    HRESULT (STDMETHODCALLTYPE* GetRequiredButtonLabel)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationController* This,
        enum __x_ABI_CWindows_CGaming_CInput_CRequiredUINavigationButtons button,
        enum __x_ABI_CWindows_CGaming_CInput_CGameControllerButtonLabel* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationController
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetOptionalButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetOptionalButtonLabel(This, button, value))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationController_GetRequiredButtonLabel(This, button, value) \
    ((This)->lpVtbl->GetRequiredButtonLabel(This, button, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationController;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationControllerStatics[] = L"Windows.Gaming.Input.IUINavigationControllerStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_UINavigationControllerAdded)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UINavigationControllerAdded)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UINavigationControllerRemoved)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        __FIEventHandler_1_Windows__CGaming__CInput__CUINavigationController* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UINavigationControllerRemoved)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_UINavigationControllers)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics* This,
        __FIVectorView_1_Windows__CGaming__CInput__CUINavigationController** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_add_UINavigationControllerAdded(This, value, token) \
    ((This)->lpVtbl->add_UINavigationControllerAdded(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_remove_UINavigationControllerAdded(This, token) \
    ((This)->lpVtbl->remove_UINavigationControllerAdded(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_add_UINavigationControllerRemoved(This, value, token) \
    ((This)->lpVtbl->add_UINavigationControllerRemoved(This, value, token))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_remove_UINavigationControllerRemoved(This, token) \
    ((This)->lpVtbl->remove_UINavigationControllerRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_get_UINavigationControllers(This, value) \
    ((This)->lpVtbl->get_UINavigationControllers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.IUINavigationControllerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.UINavigationController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.IUINavigationControllerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_IUINavigationControllerStatics2[] = L"Windows.Gaming.Input.IUINavigationControllerStatics2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIUINavigationController** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_FromGameController(This, gameController, value) \
    ((This)->lpVtbl->FromGameController(This, gameController, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CIUINavigationControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.ArcadeStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IArcadeStickStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IArcadeStickStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IArcadeStick ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ArcadeStick_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ArcadeStick_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ArcadeStick[] = L"Windows.Gaming.Input.ArcadeStick";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.FlightStick
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IFlightStickStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IFlightStick ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_FlightStick_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_FlightStick_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_FlightStick[] = L"Windows.Gaming.Input.FlightStick";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.Gamepad
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IGamepadStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IGamepadStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IGamepad ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGamepad2
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Gamepad_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Gamepad_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Gamepad[] = L"Windows.Gaming.Input.Gamepad";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.Headset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IHeadset ** Default Interface **
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Headset_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Headset_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Headset[] = L"Windows.Gaming.Input.Headset";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.RacingWheel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IRacingWheelStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IRacingWheelStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IRacingWheel ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_RacingWheel_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_RacingWheel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_RacingWheel[] = L"Windows.Gaming.Input.RacingWheel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.RawGameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IRawGameControllerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IRawGameController ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *    Windows.Gaming.Input.IRawGameController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_RawGameController_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_RawGameController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_RawGameController[] = L"Windows.Gaming.Input.RawGameController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.UINavigationController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.IUINavigationControllerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.IUINavigationControllerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.IUINavigationController ** Default Interface **
 *    Windows.Gaming.Input.IGameController
 *    Windows.Gaming.Input.IGameControllerBatteryInfo
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_UINavigationController_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_UINavigationController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_UINavigationController[] = L"Windows.Gaming.Input.UINavigationController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Einput_p_h__

#endif // __windows2Egaming2Einput_h__
