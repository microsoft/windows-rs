
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
#ifndef __windows2Esystem2Epower_h__
#define __windows2Esystem2Epower_h__
#ifndef __windows2Esystem2Epower_p_h__
#define __windows2Esystem2Epower_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                interface IBackgroundEnergyManagerStatics;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics ABI::Windows::System::Power::IBackgroundEnergyManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                interface IForegroundEnergyManagerStatics;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics ABI::Windows::System::Power::IForegroundEnergyManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                interface IPowerManagerStatics;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics ABI::Windows::System::Power::IPowerManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                typedef enum BatteryStatus : int BatteryStatus;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                typedef enum EnergySaverStatus : int EnergySaverStatus;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                typedef enum PowerSupplyStatus : int PowerSupplyStatus;
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Power.BatteryStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                enum BatteryStatus : int
                {
                    BatteryStatus_NotPresent = 0,
                    BatteryStatus_Discharging = 1,
                    BatteryStatus_Idle = 2,
                    BatteryStatus_Charging = 3,
                };
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.EnergySaverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                enum EnergySaverStatus : int
                {
                    EnergySaverStatus_Disabled = 0,
                    EnergySaverStatus_Off = 1,
                    EnergySaverStatus_On = 2,
                };
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.PowerSupplyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                enum PowerSupplyStatus : int
                {
                    PowerSupplyStatus_NotPresent = 0,
                    PowerSupplyStatus_Inadequate = 1,
                    PowerSupplyStatus_Adequate = 2,
                };
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IBackgroundEnergyManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.BackgroundEnergyManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IBackgroundEnergyManagerStatics[] = L"Windows.System.Power.IBackgroundEnergyManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                MIDL_INTERFACE("b3161d95-1180-4376-96e1-4095568147ce")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                IBackgroundEnergyManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_LowUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_NearMaxAcceptableUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAcceptableUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_ExcessiveUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_NearTerminationUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_TerminationUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_RecentEnergyUsage(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_RecentEnergyUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE add_RecentEnergyUsageIncreased(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE remove_RecentEnergyUsageIncreased(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE add_RecentEnergyUsageReturnedToLow(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE remove_RecentEnergyUsageReturnedToLow(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundEnergyManagerStatics = __uuidof(IBackgroundEnergyManagerStatics);
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IForegroundEnergyManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.ForegroundEnergyManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IForegroundEnergyManagerStatics[] = L"Windows.System.Power.IForegroundEnergyManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                MIDL_INTERFACE("9ff86872-e677-4814-9a20-5337ca732b98")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                IForegroundEnergyManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_LowUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_NearMaxAcceptableUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAcceptableUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_ExcessiveUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_RecentEnergyUsage(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE get_RecentEnergyUsageLevel(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE add_RecentEnergyUsageIncreased(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE remove_RecentEnergyUsageIncreased(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE add_RecentEnergyUsageReturnedToLow(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    virtual HRESULT STDMETHODCALLTYPE remove_RecentEnergyUsageReturnedToLow(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IForegroundEnergyManagerStatics = __uuidof(IForegroundEnergyManagerStatics);
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IPowerManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.PowerManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IPowerManagerStatics[] = L"Windows.System.Power.IPowerManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                MIDL_INTERFACE("1394825d-62ce-4364-98d5-aa28c7fbd15b")
                IPowerManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EnergySaverStatus(
                        ABI::Windows::System::Power::EnergySaverStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnergySaverStatusChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnergySaverStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BatteryStatus(
                        ABI::Windows::System::Power::BatteryStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_BatteryStatusChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BatteryStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerSupplyStatus(
                        ABI::Windows::System::Power::PowerSupplyStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PowerSupplyStatusChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PowerSupplyStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemainingChargePercent(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemainingChargePercentChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemainingChargePercentChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemainingDischargeTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemainingDischargeTimeChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemainingDischargeTimeChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPowerManagerStatics = __uuidof(IPowerManagerStatics);
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.BackgroundEnergyManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IBackgroundEnergyManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_BackgroundEnergyManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_BackgroundEnergyManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_BackgroundEnergyManager[] = L"Windows.System.Power.BackgroundEnergyManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.ForegroundEnergyManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IForegroundEnergyManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_ForegroundEnergyManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_ForegroundEnergyManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_ForegroundEnergyManager[] = L"Windows.System.Power.ForegroundEnergyManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.PowerManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IPowerManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_PowerManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_PowerManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_PowerManager[] = L"Windows.System.Power.PowerManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CPower_CBatteryStatus __x_ABI_CWindows_CSystem_CPower_CBatteryStatus;

typedef enum __x_ABI_CWindows_CSystem_CPower_CEnergySaverStatus __x_ABI_CWindows_CSystem_CPower_CEnergySaverStatus;

typedef enum __x_ABI_CWindows_CSystem_CPower_CPowerSupplyStatus __x_ABI_CWindows_CSystem_CPower_CPowerSupplyStatus;

/*
 *
 * Struct Windows.System.Power.BatteryStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CPower_CBatteryStatus
{
    BatteryStatus_NotPresent = 0,
    BatteryStatus_Discharging = 1,
    BatteryStatus_Idle = 2,
    BatteryStatus_Charging = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.EnergySaverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CPower_CEnergySaverStatus
{
    EnergySaverStatus_Disabled = 0,
    EnergySaverStatus_Off = 1,
    EnergySaverStatus_On = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.PowerSupplyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CPower_CPowerSupplyStatus
{
    PowerSupplyStatus_NotPresent = 0,
    PowerSupplyStatus_Inadequate = 1,
    PowerSupplyStatus_Adequate = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IBackgroundEnergyManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.BackgroundEnergyManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IBackgroundEnergyManagerStatics[] = L"Windows.System.Power.IBackgroundEnergyManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_LowUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_NearMaxAcceptableUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_MaxAcceptableUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_ExcessiveUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_NearTerminationUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_TerminationUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_RecentEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_RecentEnergyUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* add_RecentEnergyUsageIncreased)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* remove_RecentEnergyUsageIncreased)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* add_RecentEnergyUsageReturnedToLow)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* remove_RecentEnergyUsageReturnedToLow)(__x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_LowUsageLevel(This, value) \
    ((This)->lpVtbl->get_LowUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_NearMaxAcceptableUsageLevel(This, value) \
    ((This)->lpVtbl->get_NearMaxAcceptableUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_MaxAcceptableUsageLevel(This, value) \
    ((This)->lpVtbl->get_MaxAcceptableUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_ExcessiveUsageLevel(This, value) \
    ((This)->lpVtbl->get_ExcessiveUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_NearTerminationUsageLevel(This, value) \
    ((This)->lpVtbl->get_NearTerminationUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_TerminationUsageLevel(This, value) \
    ((This)->lpVtbl->get_TerminationUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_RecentEnergyUsage(This, value) \
    ((This)->lpVtbl->get_RecentEnergyUsage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_get_RecentEnergyUsageLevel(This, value) \
    ((This)->lpVtbl->get_RecentEnergyUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_add_RecentEnergyUsageIncreased(This, handler, token) \
    ((This)->lpVtbl->add_RecentEnergyUsageIncreased(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_remove_RecentEnergyUsageIncreased(This, token) \
    ((This)->lpVtbl->remove_RecentEnergyUsageIncreased(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_add_RecentEnergyUsageReturnedToLow(This, handler, token) \
    ((This)->lpVtbl->add_RecentEnergyUsageReturnedToLow(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_remove_RecentEnergyUsageReturnedToLow(This, token) \
    ((This)->lpVtbl->remove_RecentEnergyUsageReturnedToLow(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIBackgroundEnergyManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IForegroundEnergyManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.ForegroundEnergyManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IForegroundEnergyManagerStatics[] = L"Windows.System.Power.IForegroundEnergyManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_LowUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_NearMaxAcceptableUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_MaxAcceptableUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_ExcessiveUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_RecentEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_RecentEnergyUsageLevel)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* add_RecentEnergyUsageIncreased)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* remove_RecentEnergyUsageIncreased)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* add_RecentEnergyUsageReturnedToLow)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* remove_RecentEnergyUsageReturnedToLow)(__x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_LowUsageLevel(This, value) \
    ((This)->lpVtbl->get_LowUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_NearMaxAcceptableUsageLevel(This, value) \
    ((This)->lpVtbl->get_NearMaxAcceptableUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_MaxAcceptableUsageLevel(This, value) \
    ((This)->lpVtbl->get_MaxAcceptableUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_ExcessiveUsageLevel(This, value) \
    ((This)->lpVtbl->get_ExcessiveUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_RecentEnergyUsage(This, value) \
    ((This)->lpVtbl->get_RecentEnergyUsage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_get_RecentEnergyUsageLevel(This, value) \
    ((This)->lpVtbl->get_RecentEnergyUsageLevel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_add_RecentEnergyUsageIncreased(This, handler, token) \
    ((This)->lpVtbl->add_RecentEnergyUsageIncreased(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_remove_RecentEnergyUsageIncreased(This, token) \
    ((This)->lpVtbl->remove_RecentEnergyUsageIncreased(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_add_RecentEnergyUsageReturnedToLow(This, handler, token) \
    ((This)->lpVtbl->add_RecentEnergyUsageReturnedToLow(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_remove_RecentEnergyUsageReturnedToLow(This, token) \
    ((This)->lpVtbl->remove_RecentEnergyUsageReturnedToLow(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIForegroundEnergyManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.IPowerManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.PowerManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_IPowerManagerStatics[] = L"Windows.System.Power.IPowerManagerStatics";
typedef struct __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnergySaverStatus)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CPower_CEnergySaverStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_EnergySaverStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnergySaverStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_BatteryStatus)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CPower_CBatteryStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_BatteryStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BatteryStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_PowerSupplyStatus)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CPower_CPowerSupplyStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_PowerSupplyStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PowerSupplyStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_RemainingChargePercent)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* add_RemainingChargePercentChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemainingChargePercentChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_RemainingDischargeTime)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* add_RemainingDischargeTimeChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemainingDischargeTimeChanged)(__x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_get_EnergySaverStatus(This, value) \
    ((This)->lpVtbl->get_EnergySaverStatus(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_add_EnergySaverStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_EnergySaverStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_remove_EnergySaverStatusChanged(This, token) \
    ((This)->lpVtbl->remove_EnergySaverStatusChanged(This, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_get_BatteryStatus(This, value) \
    ((This)->lpVtbl->get_BatteryStatus(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_add_BatteryStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_BatteryStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_remove_BatteryStatusChanged(This, token) \
    ((This)->lpVtbl->remove_BatteryStatusChanged(This, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_get_PowerSupplyStatus(This, value) \
    ((This)->lpVtbl->get_PowerSupplyStatus(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_add_PowerSupplyStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_PowerSupplyStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_remove_PowerSupplyStatusChanged(This, token) \
    ((This)->lpVtbl->remove_PowerSupplyStatusChanged(This, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_get_RemainingChargePercent(This, value) \
    ((This)->lpVtbl->get_RemainingChargePercent(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_add_RemainingChargePercentChanged(This, handler, token) \
    ((This)->lpVtbl->add_RemainingChargePercentChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_remove_RemainingChargePercentChanged(This, token) \
    ((This)->lpVtbl->remove_RemainingChargePercentChanged(This, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_get_RemainingDischargeTime(This, value) \
    ((This)->lpVtbl->get_RemainingDischargeTime(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_add_RemainingDischargeTimeChanged(This, handler, token) \
    ((This)->lpVtbl->add_RemainingDischargeTimeChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_remove_RemainingDischargeTimeChanged(This, token) \
    ((This)->lpVtbl->remove_RemainingDischargeTimeChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CIPowerManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.BackgroundEnergyManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IBackgroundEnergyManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_BackgroundEnergyManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_BackgroundEnergyManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_BackgroundEnergyManager[] = L"Windows.System.Power.BackgroundEnergyManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.ForegroundEnergyManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IForegroundEnergyManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_ForegroundEnergyManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_ForegroundEnergyManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Foreground Energy Manager has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_ForegroundEnergyManager[] = L"Windows.System.Power.ForegroundEnergyManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.PowerManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.IPowerManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_PowerManager_DEFINED
#define RUNTIMECLASS_Windows_System_Power_PowerManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_PowerManager[] = L"Windows.System.Power.PowerManager";
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
#endif // __windows2Esystem2Epower_p_h__

#endif // __windows2Esystem2Epower_h__
