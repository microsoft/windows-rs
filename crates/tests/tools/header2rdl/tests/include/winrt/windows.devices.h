
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
#ifndef __windows2Edevices_h__
#define __windows2Edevices_h__
#ifndef __windows2Edevices_p_h__
#define __windows2Edevices_p_h__


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
#if !defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)
#define WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Adc.Provider.h"
#include "Windows.Devices.Gpio.Provider.h"
#include "Windows.Devices.I2c.Provider.h"
#include "Windows.Devices.Pwm.Provider.h"
#include "Windows.Devices.Spi.Provider.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            interface ILowLevelDevicesAggregateProvider;
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider ABI::Windows::Devices::ILowLevelDevicesAggregateProvider

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            interface ILowLevelDevicesAggregateProviderFactory;
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory ABI::Windows::Devices::ILowLevelDevicesAggregateProviderFactory

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            interface ILowLevelDevicesController;
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController ABI::Windows::Devices::ILowLevelDevicesController

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            interface ILowLevelDevicesControllerStatics;
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics ABI::Windows::Devices::ILowLevelDevicesControllerStatics

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                namespace Provider {
                    interface IAdcControllerProvider;
                } /* Provider */
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider ABI::Windows::Devices::Adc::Provider::IAdcControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioControllerProvider;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    interface II2cControllerProvider;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider ABI::Windows::Devices::I2c::Provider::II2cControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                namespace Provider {
                    interface IPwmControllerProvider;
                } /* Provider */
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider ABI::Windows::Devices::Pwm::Provider::IPwmControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface ISpiControllerProvider;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            class LowLevelDevicesAggregateProvider;
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesAggregateProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesAggregateProvider[] = L"Windows.Devices.ILowLevelDevicesAggregateProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            MIDL_INTERFACE("a73e561c-aac1-4ec7-a852-479f7060d01f")
            ILowLevelDevicesAggregateProvider : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AdcControllerProvider(
                    ABI::Windows::Devices::Adc::Provider::IAdcControllerProvider** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PwmControllerProvider(
                    ABI::Windows::Devices::Pwm::Provider::IPwmControllerProvider** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GpioControllerProvider(
                    ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_I2cControllerProvider(
                    ABI::Windows::Devices::I2c::Provider::II2cControllerProvider** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SpiControllerProvider(
                    ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILowLevelDevicesAggregateProvider = __uuidof(ILowLevelDevicesAggregateProvider);
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesAggregateProviderFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesAggregateProvider
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesAggregateProviderFactory[] = L"Windows.Devices.ILowLevelDevicesAggregateProviderFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            MIDL_INTERFACE("9ac4aaf6-3473-465e-96d5-36281a2c57af")
            ILowLevelDevicesAggregateProviderFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    ABI::Windows::Devices::Adc::Provider::IAdcControllerProvider* adc,
                    ABI::Windows::Devices::Pwm::Provider::IPwmControllerProvider* pwm,
                    ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider* gpio,
                    ABI::Windows::Devices::I2c::Provider::II2cControllerProvider* i2c,
                    ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider* spi,
                    ABI::Windows::Devices::ILowLevelDevicesAggregateProvider** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILowLevelDevicesAggregateProviderFactory = __uuidof(ILowLevelDevicesAggregateProviderFactory);
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesController[] = L"Windows.Devices.ILowLevelDevicesController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            MIDL_INTERFACE("2ec23dd4-179b-45de-9b39-3ae02527de52")
            ILowLevelDevicesController : public IInspectable
            {
            public:
            };

            MIDL_CONST_ID IID& IID_ILowLevelDevicesController = __uuidof(ILowLevelDevicesController);
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesControllerStatics[] = L"Windows.Devices.ILowLevelDevicesControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            MIDL_INTERFACE("093e926a-fccb-4394-a697-19de637c2db3")
            ILowLevelDevicesControllerStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DefaultProvider(
                    ABI::Windows::Devices::ILowLevelDevicesAggregateProvider** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_DefaultProvider(
                    ABI::Windows::Devices::ILowLevelDevicesAggregateProvider* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILowLevelDevicesControllerStatics = __uuidof(ILowLevelDevicesControllerStatics);
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.LowLevelDevicesAggregateProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.ILowLevelDevicesAggregateProviderFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.ILowLevelDevicesAggregateProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_LowLevelDevicesAggregateProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_LowLevelDevicesAggregateProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_LowLevelDevicesAggregateProvider[] = L"Windows.Devices.LowLevelDevicesAggregateProvider";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.LowLevelDevicesController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.ILowLevelDevicesControllerStatics interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.ILowLevelDevicesController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_LowLevelDevicesController_DEFINED
#define RUNTIMECLASS_Windows_Devices_LowLevelDevicesController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_LowLevelDevicesController[] = L"Windows.Devices.LowLevelDevicesController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider;

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory;

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CILowLevelDevicesController __x_ABI_CWindows_CDevices_CILowLevelDevicesController;

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics;

#endif // ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesAggregateProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesAggregateProvider[] = L"Windows.Devices.ILowLevelDevicesAggregateProvider";
typedef struct __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdcControllerProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_PwmControllerProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_GpioControllerProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_I2cControllerProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_SpiControllerProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderVtbl;

interface __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_get_AdcControllerProvider(This, value) \
    ((This)->lpVtbl->get_AdcControllerProvider(This, value))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_get_PwmControllerProvider(This, value) \
    ((This)->lpVtbl->get_PwmControllerProvider(This, value))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_get_GpioControllerProvider(This, value) \
    ((This)->lpVtbl->get_GpioControllerProvider(This, value))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_get_I2cControllerProvider(This, value) \
    ((This)->lpVtbl->get_I2cControllerProvider(This, value))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_get_SpiControllerProvider(This, value) \
    ((This)->lpVtbl->get_SpiControllerProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesAggregateProviderFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesAggregateProvider
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesAggregateProviderFactory[] = L"Windows.Devices.ILowLevelDevicesAggregateProviderFactory";
typedef struct __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory* This,
        __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcControllerProvider* adc,
        __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmControllerProvider* pwm,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* gpio,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* i2c,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* spi,
        __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_Create(This, adc, pwm, gpio, i2c, spi, value) \
    ((This)->lpVtbl->Create(This, adc, pwm, gpio, i2c, spi, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesController[] = L"Windows.Devices.ILowLevelDevicesController";
typedef struct __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CILowLevelDevicesController* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerVtbl;

interface __x_ABI_CWindows_CDevices_CILowLevelDevicesController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.ILowLevelDevicesControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.LowLevelDevicesController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_ILowLevelDevicesControllerStatics[] = L"Windows.Devices.ILowLevelDevicesControllerStatics";
typedef struct __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DefaultProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider** value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultProvider)(__x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics* This,
        __x_ABI_CWindows_CDevices_CILowLevelDevicesAggregateProvider* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_get_DefaultProvider(This, value) \
    ((This)->lpVtbl->get_DefaultProvider(This, value))

#define __x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_put_DefaultProvider(This, value) \
    ((This)->lpVtbl->put_DefaultProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CILowLevelDevicesControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.LowLevelDevicesAggregateProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.ILowLevelDevicesAggregateProviderFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.ILowLevelDevicesAggregateProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_LowLevelDevicesAggregateProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_LowLevelDevicesAggregateProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_LowLevelDevicesAggregateProvider[] = L"Windows.Devices.LowLevelDevicesAggregateProvider";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.LowLevelDevicesController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.ILowLevelDevicesControllerStatics interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.ILowLevelDevicesController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_LowLevelDevicesController_DEFINED
#define RUNTIMECLASS_Windows_Devices_LowLevelDevicesController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_LowLevelDevicesController[] = L"Windows.Devices.LowLevelDevicesController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices_p_h__

#endif // __windows2Edevices_h__
