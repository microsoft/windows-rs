
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
#ifndef __windows2Edevices2Egpio2Eprovider_h__
#define __windows2Edevices2Egpio2Eprovider_h__
#ifndef __windows2Edevices2Egpio2Eprovider_p_h__
#define __windows2Edevices2Egpio2Eprovider_p_h__


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
#include "Windows.Devices.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioPinProvider;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider ABI::Windows::Devices::Gpio::Provider::IGpioPinProvider

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioPinProviderValueChangedEventArgs;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs ABI::Windows::Devices::Gpio::Provider::IGpioPinProviderValueChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioPinProviderValueChangedEventArgsFactory;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory ABI::Windows::Devices::Gpio::Provider::IGpioPinProviderValueChangedEventArgsFactory

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioProvider;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider ABI::Windows::Devices::Gpio::Provider::IGpioProvider

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#define DEF___FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6ac0edb9-e3c9-5840-8aa8-1bc45366f6ca"))
IIterator<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> : IIterator_impl<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Gpio.Provider.IGpioControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t;
#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#define DEF___FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09212bd4-851b-52bd-b82c-421bf3d6f511"))
IIterable<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> : IIterable_impl<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Gpio.Provider.IGpioControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t;
#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f429355f-7a16-5dcf-a575-db7d2a20eced"))
IVectorView<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> : IVectorView_impl<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Gpio.Provider.IGpioControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Gpio::Provider::IGpioControllerProvider*> __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t;
#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    class GpioPinProviderValueChangedEventArgs;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af259d89-9e01-529e-a879-c6763142d160"))
ITypedEventHandler<ABI::Windows::Devices::Gpio::Provider::IGpioPinProvider*, ABI::Windows::Devices::Gpio::Provider::GpioPinProviderValueChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Devices::Gpio::Provider::IGpioPinProvider*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::Provider::GpioPinProviderValueChangedEventArgs*, ABI::Windows::Devices::Gpio::Provider::IGpioPinProviderValueChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Gpio.Provider.IGpioPinProvider, Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Gpio::Provider::IGpioPinProvider*, ABI::Windows::Devices::Gpio::Provider::GpioPinProviderValueChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    typedef enum ProviderGpioPinDriveMode : int ProviderGpioPinDriveMode;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    typedef enum ProviderGpioPinEdge : int ProviderGpioPinEdge;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    typedef enum ProviderGpioPinValue : int ProviderGpioPinValue;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    typedef enum ProviderGpioSharingMode : int ProviderGpioSharingMode;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinDriveMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    enum ProviderGpioPinDriveMode : int
                    {
                        ProviderGpioPinDriveMode_Input = 0,
                        ProviderGpioPinDriveMode_Output = 1,
                        ProviderGpioPinDriveMode_InputPullUp = 2,
                        ProviderGpioPinDriveMode_InputPullDown = 3,
                        ProviderGpioPinDriveMode_OutputOpenDrain = 4,
                        ProviderGpioPinDriveMode_OutputOpenDrainPullUp = 5,
                        ProviderGpioPinDriveMode_OutputOpenSource = 6,
                        ProviderGpioPinDriveMode_OutputOpenSourcePullDown = 7,
                    };
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinEdge
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    enum ProviderGpioPinEdge : int
                    {
                        ProviderGpioPinEdge_FallingEdge = 0,
                        ProviderGpioPinEdge_RisingEdge = 1,
                    };
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinValue
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    enum ProviderGpioPinValue : int
                    {
                        ProviderGpioPinValue_Low = 0,
                        ProviderGpioPinValue_High = 1,
                    };
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    enum ProviderGpioSharingMode : int
                    {
                        ProviderGpioSharingMode_Exclusive = 0,
                        ProviderGpioSharingMode_SharedReadOnly = 1,
                    };
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioControllerProvider[] = L"Windows.Devices.Gpio.Provider.IGpioControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    MIDL_INTERFACE("ad11cec7-19ea-4b21-874f-b91aed4a25db")
                    IGpioControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PinCount(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OpenPinProvider(
                            INT32 pin,
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioSharingMode sharingMode,
                            ABI::Windows::Devices::Gpio::Provider::IGpioPinProvider** gpioPinProvider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGpioControllerProvider = __uuidof(IGpioControllerProvider);
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProvider[] = L"Windows.Devices.Gpio.Provider.IGpioPinProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    MIDL_INTERFACE("42344cb7-6abc-40ff-9ce7-73b85301b900")
                    IGpioPinProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_ValueChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ValueChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DebounceTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DebounceTimeout(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PinNumber(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SharingMode(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioSharingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsDriveModeSupported(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinDriveMode driveMode,
                            boolean* supported
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDriveMode(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinDriveMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDriveMode(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinDriveMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Write(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinValue value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Read(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinValue* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGpioPinProvider = __uuidof(IGpioPinProvider);
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProviderValueChangedEventArgs[] = L"Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    MIDL_INTERFACE("32a6d6f2-3d5b-44cd-8fbe-13a69f2edb24")
                    IGpioPinProviderValueChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Edge(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinEdge* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGpioPinProviderValueChangedEventArgs = __uuidof(IGpioPinProviderValueChangedEventArgs);
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProviderValueChangedEventArgsFactory[] = L"Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    MIDL_INTERFACE("3ecb0b59-568c-4392-b24a-8a59a902b1f1")
                    IGpioPinProviderValueChangedEventArgsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::Devices::Gpio::Provider::ProviderGpioPinEdge edge,
                            ABI::Windows::Devices::Gpio::Provider::IGpioPinProviderValueChangedEventArgs** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGpioPinProviderValueChangedEventArgsFactory = __uuidof(IGpioPinProviderValueChangedEventArgsFactory);
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioProvider[] = L"Windows.Devices.Gpio.Provider.IGpioProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    MIDL_INTERFACE("44e82707-08ca-434a-afe0-d61580446f7e")
                    IGpioProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetControllers(
                            __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGpioProvider = __uuidof(IGpioProvider);
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs[] = L"Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

typedef struct __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl;

interface __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

typedef struct __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        __FIIterator_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl;

interface __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider;

typedef struct __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl;

interface __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* sender,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinEdge __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinEdge;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinValue __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinValue;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioSharingMode __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioSharingMode;

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinDriveMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode
{
    ProviderGpioPinDriveMode_Input = 0,
    ProviderGpioPinDriveMode_Output = 1,
    ProviderGpioPinDriveMode_InputPullUp = 2,
    ProviderGpioPinDriveMode_InputPullDown = 3,
    ProviderGpioPinDriveMode_OutputOpenDrain = 4,
    ProviderGpioPinDriveMode_OutputOpenDrainPullUp = 5,
    ProviderGpioPinDriveMode_OutputOpenSource = 6,
    ProviderGpioPinDriveMode_OutputOpenSourcePullDown = 7,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinEdge
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinEdge
{
    ProviderGpioPinEdge_FallingEdge = 0,
    ProviderGpioPinEdge_RisingEdge = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioPinValue
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinValue
{
    ProviderGpioPinValue_Low = 0,
    ProviderGpioPinValue_High = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Gpio.Provider.ProviderGpioSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioSharingMode
{
    ProviderGpioSharingMode_Exclusive = 0,
    ProviderGpioSharingMode_SharedReadOnly = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioControllerProvider[] = L"Windows.Devices.Gpio.Provider.IGpioControllerProvider";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinCount)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* OpenPinProvider)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider* This,
        INT32 pin,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioSharingMode sharingMode,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider** gpioPinProvider);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProviderVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_get_PinCount(This, value) \
    ((This)->lpVtbl->get_PinCount(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_OpenPinProvider(This, pin, sharingMode, gpioPinProvider) \
    ((This)->lpVtbl->OpenPinProvider(This, pin, sharingMode, gpioPinProvider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProvider[] = L"Windows.Devices.Gpio.Provider.IGpioPinProvider";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ValueChanged)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        __FITypedEventHandler_2_Windows__CDevices__CGpio__CProvider__CIGpioPinProvider_Windows__CDevices__CGpio__CProvider__CGpioPinProviderValueChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ValueChanged)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_DebounceTimeout)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DebounceTimeout)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_PinNumber)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SharingMode)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioSharingMode* value);
    HRESULT (STDMETHODCALLTYPE* IsDriveModeSupported)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode driveMode,
        boolean* supported);
    HRESULT (STDMETHODCALLTYPE* GetDriveMode)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode* value);
    HRESULT (STDMETHODCALLTYPE* SetDriveMode)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinDriveMode value);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinValue value);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinValue* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_add_ValueChanged(This, handler, token) \
    ((This)->lpVtbl->add_ValueChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_remove_ValueChanged(This, token) \
    ((This)->lpVtbl->remove_ValueChanged(This, token))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_get_DebounceTimeout(This, value) \
    ((This)->lpVtbl->get_DebounceTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_put_DebounceTimeout(This, value) \
    ((This)->lpVtbl->put_DebounceTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_get_PinNumber(This, value) \
    ((This)->lpVtbl->get_PinNumber(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_get_SharingMode(This, value) \
    ((This)->lpVtbl->get_SharingMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_IsDriveModeSupported(This, driveMode, supported) \
    ((This)->lpVtbl->IsDriveModeSupported(This, driveMode, supported))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_GetDriveMode(This, value) \
    ((This)->lpVtbl->GetDriveMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_SetDriveMode(This, value) \
    ((This)->lpVtbl->SetDriveMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_Write(This, value) \
    ((This)->lpVtbl->Write(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_Read(This, value) \
    ((This)->lpVtbl->Read(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProviderValueChangedEventArgs[] = L"Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Edge)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinEdge* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_get_Edge(This, value) \
    ((This)->lpVtbl->get_Edge(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioPinProviderValueChangedEventArgsFactory[] = L"Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CProvider_CProviderGpioPinEdge edge,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgs** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_Create(This, edge, value) \
    ((This)->lpVtbl->Create(This, edge, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioPinProviderValueChangedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.Provider.IGpioProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_Provider_IGpioProvider[] = L"Windows.Devices.Gpio.Provider.IGpioProvider";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllers)(__x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* This,
        __FIVectorView_1_Windows__CDevices__CGpio__CProvider__CIGpioControllerProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProviderVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_GetControllers(This, result) \
    ((This)->lpVtbl->GetControllers(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_Provider_GpioPinProviderValueChangedEventArgs[] = L"Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs";
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
#endif // __windows2Edevices2Egpio2Eprovider_p_h__

#endif // __windows2Edevices2Egpio2Eprovider_h__
