
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
#ifndef __windows2Edevices2Epower_h__
#define __windows2Edevices2Epower_h__
#ifndef __windows2Edevices2Epower_p_h__
#define __windows2Edevices2Epower_p_h__


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
#include "Windows.System.Power.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IBattery;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIBattery ABI::Windows::Devices::Power::IBattery

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IBatteryStatics;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics ABI::Windows::Devices::Power::IBatteryStatics

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IPowerGridData;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData ABI::Windows::Devices::Power::IPowerGridData

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IPowerGridForecast;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast ABI::Windows::Devices::Power::IPowerGridForecast

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                interface IPowerGridForecastStatics;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics ABI::Windows::Devices::Power::IPowerGridForecastStatics

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                class Battery;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("daa3d556-1529-56d2-a5f8-bfb6c22a3dfe"))
IAsyncOperation<ABI::Windows::Devices::Power::Battery*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::Battery*, ABI::Windows::Devices::Power::IBattery*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Power.Battery>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Power::Battery*> __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_t;
#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("97f82115-3822-507b-82e6-2777b336e98e"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Power::Battery*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::Battery*, ABI::Windows::Devices::Power::IBattery*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Power.Battery>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Power::Battery*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                class PowerGridData;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CPower__CPowerGridData_USE
#define DEF___FIIterator_1_Windows__CDevices__CPower__CPowerGridData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("96f5ff4c-64f3-5eab-b36a-48283ddda59f"))
IIterator<ABI::Windows::Devices::Power::PowerGridData*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::PowerGridData*, ABI::Windows::Devices::Power::IPowerGridData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Power.PowerGridData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Power::PowerGridData*> __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_t;
#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CPower__CPowerGridData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CPower__CPowerGridData_USE */

#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CPower__CPowerGridData_USE
#define DEF___FIIterable_1_Windows__CDevices__CPower__CPowerGridData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad396c1e-0752-5eab-b2e8-f4da8f31d9ba"))
IIterable<ABI::Windows::Devices::Power::PowerGridData*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::PowerGridData*, ABI::Windows::Devices::Power::IPowerGridData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Power.PowerGridData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Power::PowerGridData*> __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_t;
#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CPower__CPowerGridData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CPower__CPowerGridData_USE */

#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_USE
#define DEF___FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e204ee85-7062-5207-ba91-7103384d5b97"))
IVectorView<ABI::Windows::Devices::Power::PowerGridData*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::PowerGridData*, ABI::Windows::Devices::Power::IPowerGridData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Power.PowerGridData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Power::PowerGridData*> __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_t;
#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_USE */

#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4d4aa646-767f-5645-af5c-546464d3ec09"))
ITypedEventHandler<ABI::Windows::Devices::Power::Battery*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Power::Battery*, ABI::Windows::Devices::Power::IBattery*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Power.Battery, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Power::Battery*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace Devices {
            namespace Power {
                class BatteryReport;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                class PowerGridForecast;
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Devices.Power.IBattery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.Battery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBattery[] = L"Windows.Devices.Power.IBattery";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("bc894fc6-0072-47c8-8b5d-614aaa7a437e")
                IBattery : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::Devices::Power::IBatteryReport** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ReportUpdated(
                        __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ReportUpdated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBattery = __uuidof(IBattery);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBattery;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IBatteryReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.BatteryReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBatteryReport[] = L"Windows.Devices.Power.IBatteryReport";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("c9858c3a-4e13-420a-a8d0-24f18f395401")
                IBatteryReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChargeRateInMilliwatts(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesignCapacityInMilliwattHours(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FullChargeCapacityInMilliwattHours(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemainingCapacityInMilliwattHours(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::System::Power::BatteryStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBatteryReport = __uuidof(IBatteryReport);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBatteryReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IBatteryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.Battery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBatteryStatics[] = L"Windows.Devices.Power.IBatteryStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("79cd72b6-9e5e-4452-bea6-dfcd541e597f")
                IBatteryStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AggregateBattery(
                        ABI::Windows::Devices::Power::IBattery** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBatteryStatics = __uuidof(IBatteryStatics);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBatteryStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridData
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridData
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridData[] = L"Windows.Devices.Power.IPowerGridData";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("c360fb17-fc92-5f6e-999d-16a4cf9d6c40")
                IPowerGridData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Severity(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLowUserExperienceImpact(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPowerGridData = __uuidof(IPowerGridData);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridForecast
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridForecast
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridForecast[] = L"Windows.Devices.Power.IPowerGridForecast";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("077e4de9-ed60-58bb-a850-003c6a138685")
                IPowerGridForecast : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BlockDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Forecast(
                        __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPowerGridForecast = __uuidof(IPowerGridForecast);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridForecastStatics
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridForecast
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridForecastStatics[] = L"Windows.Devices.Power.IPowerGridForecastStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Power {
                MIDL_INTERFACE("5b78c806-2e4e-5bcc-bb34-cb81c60f9e12")
                IPowerGridForecastStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForecast(
                        ABI::Windows::Devices::Power::IPowerGridForecast** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ForecastUpdated(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ForecastUpdated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPowerGridForecastStatics = __uuidof(IPowerGridForecastStatics);
            } /* Power */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.Battery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Power.IBatteryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IBattery ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_Battery_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_Battery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_Battery[] = L"Windows.Devices.Power.Battery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.BatteryReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IBatteryReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_BatteryReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_BatteryReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_BatteryReport[] = L"Windows.Devices.Power.BatteryReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.PowerGridData
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IPowerGridData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_PowerGridData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_PowerGridData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_PowerGridData[] = L"Windows.Devices.Power.PowerGridData";
#endif
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.PowerGridForecast
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Power.IPowerGridForecastStatics interface starting with version 1.0 of the Windows.Devices.Power.PowerGridApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IPowerGridForecast ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_PowerGridForecast_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_PowerGridForecast_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_PowerGridForecast[] = L"Windows.Devices.Power.PowerGridForecast";
#endif
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIBattery __x_ABI_CWindows_CDevices_CPower_CIBattery;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBattery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIBatteryReport __x_ABI_CWindows_CDevices_CPower_CIBatteryReport;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridData __x_ABI_CWindows_CDevices_CPower_CIPowerGridData;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics;

#endif // ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CPower__CBattery;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CPower__CBatteryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* This,
        __x_ABI_CWindows_CDevices_CPower_CIBattery** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CPower__CBatteryVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CPower__CBatteryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBatteryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery* This,
        __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBatteryVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBatteryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPower__CBattery_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CPower__CPowerGridData __FIIterator_1_Windows__CDevices__CPower__CPowerGridData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CPower__CPowerGridData;

typedef struct __FIIterator_1_Windows__CDevices__CPower__CPowerGridDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridData** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CPower__CPowerGridData* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridData** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CPower__CPowerGridDataVtbl;

interface __FIIterator_1_Windows__CDevices__CPower__CPowerGridData
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CPower__CPowerGridDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CPower__CPowerGridData_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CPower__CPowerGridData __FIIterable_1_Windows__CDevices__CPower__CPowerGridData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CPower__CPowerGridData;

typedef struct __FIIterable_1_Windows__CDevices__CPower__CPowerGridDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CPower__CPowerGridData* This,
        __FIIterator_1_Windows__CDevices__CPower__CPowerGridData** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CPower__CPowerGridDataVtbl;

interface __FIIterable_1_Windows__CDevices__CPower__CPowerGridData
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CPower__CPowerGridDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CPower__CPowerGridData_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CPower__CPowerGridData;

typedef struct __FIVectorView_1_Windows__CDevices__CPower__CPowerGridDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridData** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridData* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CPower__CPowerGridData* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridData** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CPower__CPowerGridDataVtbl;

interface __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CPower__CPowerGridDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CPower__CPowerGridData_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

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

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* This,
        __x_ABI_CWindows_CDevices_CPower_CIBattery* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CPower_CBatteryStatus __x_ABI_CWindows_CSystem_CPower_CBatteryStatus;

/*
 *
 * Interface Windows.Devices.Power.IBattery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.Battery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBattery[] = L"Windows.Devices.Power.IBattery";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIBatteryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        __x_ABI_CWindows_CDevices_CPower_CIBatteryReport** result);
    HRESULT (STDMETHODCALLTYPE* add_ReportUpdated)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        __FITypedEventHandler_2_Windows__CDevices__CPower__CBattery_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ReportUpdated)(__x_ABI_CWindows_CDevices_CPower_CIBattery* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIBatteryVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIBattery
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIBatteryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_GetReport(This, result) \
    ((This)->lpVtbl->GetReport(This, result))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_add_ReportUpdated(This, handler, token) \
    ((This)->lpVtbl->add_ReportUpdated(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPower_CIBattery_remove_ReportUpdated(This, token) \
    ((This)->lpVtbl->remove_ReportUpdated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBattery;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBattery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IBatteryReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.BatteryReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBatteryReport[] = L"Windows.Devices.Power.IBatteryReport";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIBatteryReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChargeRateInMilliwatts)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_DesignCapacityInMilliwattHours)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_FullChargeCapacityInMilliwattHours)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_RemainingCapacityInMilliwattHours)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CPower_CIBatteryReport* This,
        enum __x_ABI_CWindows_CSystem_CPower_CBatteryStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIBatteryReportVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIBatteryReport
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIBatteryReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_get_ChargeRateInMilliwatts(This, value) \
    ((This)->lpVtbl->get_ChargeRateInMilliwatts(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_get_DesignCapacityInMilliwattHours(This, value) \
    ((This)->lpVtbl->get_DesignCapacityInMilliwattHours(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_get_FullChargeCapacityInMilliwattHours(This, value) \
    ((This)->lpVtbl->get_FullChargeCapacityInMilliwattHours(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_get_RemainingCapacityInMilliwattHours(This, value) \
    ((This)->lpVtbl->get_RemainingCapacityInMilliwattHours(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryReport_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBatteryReport;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IBatteryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.Battery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IBatteryStatics[] = L"Windows.Devices.Power.IBatteryStatics";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIBatteryStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AggregateBattery)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        __x_ABI_CWindows_CDevices_CPower_CIBattery** result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CPower__CBattery** result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CPower_CIBatteryStatics* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIBatteryStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIBatteryStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_get_AggregateBattery(This, result) \
    ((This)->lpVtbl->get_AggregateBattery(This, result))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_FromIdAsync(This, deviceId, result) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, result))

#define __x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIBatteryStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIBatteryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridData
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridData
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridData[] = L"Windows.Devices.Power.IPowerGridData";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Severity)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLowUserExperienceImpact)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridData* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIPowerGridDataVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridData
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_get_Severity(This, value) \
    ((This)->lpVtbl->get_Severity(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridData_get_IsLowUserExperienceImpact(This, value) \
    ((This)->lpVtbl->get_IsLowUserExperienceImpact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridData_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridForecast
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridForecast
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridForecast[] = L"Windows.Devices.Power.IPowerGridForecast";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_BlockDuration)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Forecast)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast* This,
        __FIVectorView_1_Windows__CDevices__CPower__CPowerGridData** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_get_BlockDuration(This, value) \
    ((This)->lpVtbl->get_BlockDuration(This, value))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_get_Forecast(This, value) \
    ((This)->lpVtbl->get_Forecast(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Power.IPowerGridForecastStatics
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Power.PowerGridForecast
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Power_IPowerGridForecastStatics[] = L"Windows.Devices.Power.IPowerGridForecastStatics";
typedef struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForecast)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecast** result);
    HRESULT (STDMETHODCALLTYPE* add_ForecastUpdated)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ForecastUpdated)(__x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_GetForecast(This, result) \
    ((This)->lpVtbl->GetForecast(This, result))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_add_ForecastUpdated(This, handler, token) \
    ((This)->lpVtbl->add_ForecastUpdated(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_remove_ForecastUpdated(This, token) \
    ((This)->lpVtbl->remove_ForecastUpdated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPower_CIPowerGridForecastStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.Battery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Power.IBatteryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IBattery ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_Battery_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_Battery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_Battery[] = L"Windows.Devices.Power.Battery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.BatteryReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IBatteryReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_BatteryReport_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_BatteryReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_BatteryReport[] = L"Windows.Devices.Power.BatteryReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.PowerGridData
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IPowerGridData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_PowerGridData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_PowerGridData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_PowerGridData[] = L"Windows.Devices.Power.PowerGridData";
#endif
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Power.PowerGridForecast
 *
 * Introduced to Windows.Devices.Power.PowerGridApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Power.IPowerGridForecastStatics interface starting with version 1.0 of the Windows.Devices.Power.PowerGridApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Power.IPowerGridForecast ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Power_PowerGridForecast_DEFINED
#define RUNTIMECLASS_Windows_Devices_Power_PowerGridForecast_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Power_PowerGridForecast[] = L"Windows.Devices.Power.PowerGridForecast";
#endif
#endif // WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Epower_p_h__

#endif // __windows2Edevices2Epower_h__
