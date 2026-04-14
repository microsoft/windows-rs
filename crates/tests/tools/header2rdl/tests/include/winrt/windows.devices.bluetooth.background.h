
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
#ifndef __windows2Edevices2Ebluetooth2Ebackground_h__
#define __windows2Edevices2Ebluetooth2Ebackground_h__
#ifndef __windows2Edevices2Ebluetooth2Ebackground_p_h__
#define __windows2Edevices2Ebluetooth2Ebackground_p_h__


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

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Bluetooth.h"
#include "Windows.Devices.Bluetooth.Advertisement.h"
#include "Windows.Devices.Bluetooth.GenericAttributeProfile.h"
#include "Windows.Devices.Bluetooth.Rfcomm.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IBluetoothLEAdvertisementPublisherTriggerDetails;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails ABI::Windows::Devices::Bluetooth::Background::IBluetoothLEAdvertisementPublisherTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IBluetoothLEAdvertisementPublisherTriggerDetails2;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2 ABI::Windows::Devices::Bluetooth::Background::IBluetoothLEAdvertisementPublisherTriggerDetails2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IBluetoothLEAdvertisementWatcherTriggerDetails;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails ABI::Windows::Devices::Bluetooth::Background::IBluetoothLEAdvertisementWatcherTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattCharacteristicNotificationTriggerDetails;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails ABI::Windows::Devices::Bluetooth::Background::IGattCharacteristicNotificationTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattCharacteristicNotificationTriggerDetails2;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2 ABI::Windows::Devices::Bluetooth::Background::IGattCharacteristicNotificationTriggerDetails2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattServiceProviderConnection;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnection

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattServiceProviderConnection2;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2 ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnection2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattServiceProviderConnectionStatics;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnectionStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IGattServiceProviderTriggerDetails;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IRfcommConnectionTriggerDetails;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails ABI::Windows::Devices::Bluetooth::Background::IRfcommConnectionTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IRfcommInboundConnectionInformation;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation ABI::Windows::Devices::Bluetooth::Background::IRfcommInboundConnectionInformation

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    interface IRfcommOutboundConnectionInformation;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation ABI::Windows::Devices::Bluetooth::Background::IRfcommOutboundConnectionInformation

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementReceivedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementReceivedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("096edbb8-ecef-5724-be62-240dcff6aca9"))
IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("34f6412f-8314-5205-967c-db357c9a42a7"))
IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattValueChangedEventArgs;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattValueChangedEventArgs;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattValueChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cd20a796-aa22-521d-8e0f-fc6d4a18e287"))
IIterator<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattValueChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("526a63df-8827-51b9-9e2c-9d65021a79d7"))
IIterable<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattValueChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    class GattServiceProviderConnection;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c0246566-6d14-5ab2-8443-7439a1fc16e2"))
IKeyValuePair<HSTRING, ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*, ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Devices.Bluetooth.Background.GattServiceProviderConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*> __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6faf610e-cf1e-50e4-b1cc-f322fcf25036"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Devices.Bluetooth.Background.GattServiceProviderConnection>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb2cecfb-dd6c-5329-8e17-7fb80a84191f"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Devices.Bluetooth.Background.GattServiceProviderConnection>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#define DEF___FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94eccb06-19ad-5e4f-953e-12c4aec054e9"))
IMapView<HSTRING, ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*, ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Devices.Bluetooth.Background.GattServiceProviderConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Devices::Bluetooth::Background::GattServiceProviderConnection*> __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t;
#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8aef9bca-fe7d-5966-9789-fede24cb41c4"))
IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f96e85f-ca51-5303-bd1b-acec7773baf6"))
IVectorView<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattValueChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs*> __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_short_USE
#define DEF___FIReference_1_short_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ec9e41b-6709-5647-9918-a1270110fc4e"))
IReference<short> : IReference_impl<short>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<short> __FIReference_1_short_t;
#define __FIReference_1_short ABI::Windows::Foundation::__FIReference_1_short_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_short_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    typedef enum BluetoothLEAdvertisementPublisherStatus : int BluetoothLEAdvertisementPublisherStatus;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice ABI::Windows::Devices::Bluetooth::IBluetoothDevice

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothError : int BluetoothError;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothServiceCapabilities : unsigned int BluetoothServiceCapabilities;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothSignalStrengthFilter;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothSignalStrengthFilter;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter ABI::Windows::Devices::Bluetooth::IBluetoothSignalStrengthFilter

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattCharacteristic;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattCharacteristic;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattCharacteristic

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattLocalService;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattLocalService;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattLocalService

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattServiceProviderAdvertisingParameters;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattServiceProviderAdvertisingParameters;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattServiceProviderAdvertisingParameters

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommServiceId;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommServiceId;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__

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
        namespace Networking {
            namespace Sockets {
                class StreamSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket ABI::Windows::Networking::Sockets::IStreamSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    typedef enum BluetoothEventTriggeringMode : int BluetoothEventTriggeringMode;
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    enum BluetoothEventTriggeringMode : int
                    {
                        BluetoothEventTriggeringMode_Serial = 0,
                        BluetoothEventTriggeringMode_Batch = 1,
                        BluetoothEventTriggeringMode_KeepLatest = 2,
                    };
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementPublisherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("610eca86-3480-41c9-a918-7ddadf207e00")
                    IBluetoothLEAdvertisementPublisherTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisherTriggerDetails = __uuidof(IBluetoothLEAdvertisementPublisherTriggerDetails);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementPublisherTriggerDetails2[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("d4a3d025-c601-42d6-9829-4ccb3f5cd77f")
                    IBluetoothLEAdvertisementPublisherTriggerDetails2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SelectedTransmitPowerLevelInDBm(
                            __FIReference_1_short** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisherTriggerDetails2 = __uuidof(IBluetoothLEAdvertisementPublisherTriggerDetails2);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementWatcherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("a7db5ad7-2257-4e69-9784-fee645c1dce0")
                    IBluetoothLEAdvertisementWatcherTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Advertisements(
                            __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SignalStrengthFilter(
                            ABI::Windows::Devices::Bluetooth::IBluetoothSignalStrengthFilter** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcherTriggerDetails = __uuidof(IBluetoothLEAdvertisementWatcherTriggerDetails);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattCharacteristicNotificationTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("9ba03b18-0fec-436a-93b1-f46c697532a2")
                    IGattCharacteristicNotificationTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Characteristic(
                            ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattCharacteristic** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattCharacteristicNotificationTriggerDetails = __uuidof(IGattCharacteristicNotificationTriggerDetails);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattCharacteristicNotificationTriggerDetails2[] = L"Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("727a50dc-949d-454a-b192-983467e3d50f")
                    IGattCharacteristicNotificationTriggerDetails2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EventTriggeringMode(
                            ABI::Windows::Devices::Bluetooth::Background::BluetoothEventTriggeringMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValueChangedEvents(
                            __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattCharacteristicNotificationTriggerDetails2 = __uuidof(IGattCharacteristicNotificationTriggerDetails2);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnection[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("7fa1b9b9-2f13-40b5-9582-8eb78e98ef13")
                    IGattServiceProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TriggerId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Service(
                            ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattLocalService** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattServiceProviderConnection = __uuidof(IGattServiceProviderConnection);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnection2[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("90d12be0-ebc0-484f-ae0a-7eb8b6266bac")
                    IGattServiceProviderConnection2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE UpdateAdvertisingParameters(
                            ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattServiceProviderAdvertisingParameters* parameters
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattServiceProviderConnection2 = __uuidof(IGattServiceProviderConnection2);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnectionStatics[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("3d509f4b-0b0e-4466-b8cd-6ebdda1fa17d")
                    IGattServiceProviderConnectionStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllServices(
                            __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattServiceProviderConnectionStatics = __uuidof(IGattServiceProviderConnectionStatics);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("ae8c0625-05ff-4afb-b16a-de95f3cf0158")
                    IGattServiceProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::Devices::Bluetooth::Background::IGattServiceProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGattServiceProviderTriggerDetails = __uuidof(IGattServiceProviderTriggerDetails);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommConnectionTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("f922734d-2e3c-4efc-ab59-fc5cf96f97e3")
                    IRfcommConnectionTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Socket(
                            ABI::Windows::Networking::Sockets::IStreamSocket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Incoming(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RemoteDevice(
                            ABI::Windows::Devices::Bluetooth::IBluetoothDevice** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommConnectionTriggerDetails = __uuidof(IRfcommConnectionTriggerDetails);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommInboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("6d3e75a8-5429-4059-92e3-1e8b65528707")
                    IRfcommInboundConnectionInformation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SdpRecord(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SdpRecord(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_LocalServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceCapabilities(
                            ABI::Windows::Devices::Bluetooth::BluetoothServiceCapabilities* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServiceCapabilities(
                            ABI::Windows::Devices::Bluetooth::BluetoothServiceCapabilities value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommInboundConnectionInformation = __uuidof(IRfcommInboundConnectionInformation);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommOutboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Background {
                    MIDL_INTERFACE("b091227b-f434-4cb0-99b1-4ab8cedaedd7")
                    IRfcommOutboundConnectionInformation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RemoteServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RemoteServiceId(
                            ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRfcommOutboundConnectionInformation = __uuidof(IRfcommOutboundConnectionInformation);
                } /* Background */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection[] = L"Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2 __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2 __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2 __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection;

typedef struct __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING key,
        __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection* This,
        __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** first,
        __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl;

interface __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattValueChangedEventArgs** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_short_INTERFACE_DEFINED__)
#define ____FIReference_1_short_INTERFACE_DEFINED__

typedef interface __FIReference_1_short __FIReference_1_short;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_short;

typedef struct __FIReference_1_shortVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_short* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_short* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_short* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_short* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_short* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_short* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_short* This,
        INT16* result);

    END_INTERFACE
} __FIReference_1_shortVtbl;

interface __FIReference_1_short
{
    CONST_VTBL struct __FIReference_1_shortVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_short_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_short_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_short_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_short_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_short_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_short_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_short_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_short_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus;

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities;

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CBluetoothEventTriggeringMode __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CBluetoothEventTriggeringMode;

/*
 *
 * Struct Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CBluetoothEventTriggeringMode
{
    BluetoothEventTriggeringMode_Serial = 0,
    BluetoothEventTriggeringMode_Batch = 1,
    BluetoothEventTriggeringMode_KeepLatest = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementPublisherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementPublisherTriggerDetails2[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedTransmitPowerLevelInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2* This,
        __FIReference_1_short** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_get_SelectedTransmitPowerLevelInDBm(This, value) \
    ((This)->lpVtbl->get_SelectedTransmitPowerLevelInDBm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementPublisherTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IBluetoothLEAdvertisementWatcherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);
    HRESULT (STDMETHODCALLTYPE* get_Advertisements)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs** value);
    HRESULT (STDMETHODCALLTYPE* get_SignalStrengthFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_get_Advertisements(This, value) \
    ((This)->lpVtbl->get_Advertisements(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_get_SignalStrengthFilter(This, value) \
    ((This)->lpVtbl->get_SignalStrengthFilter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIBluetoothLEAdvertisementWatcherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattCharacteristicNotificationTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Characteristic)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattCharacteristic** value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_get_Characteristic(This, value) \
    ((This)->lpVtbl->get_Characteristic(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattCharacteristicNotificationTriggerDetails2[] = L"Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);
    HRESULT (STDMETHODCALLTYPE* get_EventTriggeringMode)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CBluetoothEventTriggeringMode* value);
    HRESULT (STDMETHODCALLTYPE* get_ValueChangedEvents)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattValueChangedEventArgs** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_get_EventTriggeringMode(This, value) \
    ((This)->lpVtbl->get_EventTriggeringMode(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_get_ValueChangedEvents(This, value) \
    ((This)->lpVtbl->get_ValueChangedEvents(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattCharacteristicNotificationTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnection[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TriggerId)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Service)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattLocalService** value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_get_TriggerId(This, value) \
    ((This)->lpVtbl->get_TriggerId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_get_Service(This, value) \
    ((This)->lpVtbl->get_Service(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnection2[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UpdateAdvertisingParameters)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattServiceProviderAdvertisingParameters* parameters);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_UpdateAdvertisingParameters(This, parameters) \
    ((This)->lpVtbl->UpdateAdvertisingParameters(This, parameters))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderConnectionStatics[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllServices)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics* This,
        __FIMapView_2_HSTRING_Windows__CDevices__CBluetooth__CBackground__CGattServiceProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_get_AllServices(This, value) \
    ((This)->lpVtbl->get_AllServices(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IGattServiceProviderTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIGattServiceProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommConnectionTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Socket)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocket** value);
    HRESULT (STDMETHODCALLTYPE* get_Incoming)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_get_Socket(This, value) \
    ((This)->lpVtbl->get_Socket(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_get_Incoming(This, value) \
    ((This)->lpVtbl->get_Incoming(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_get_RemoteDevice(This, value) \
    ((This)->lpVtbl->get_RemoteDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommConnectionTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommInboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SdpRecord)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_SdpRecord)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** value);
    HRESULT (STDMETHODCALLTYPE* put_LocalServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceCapabilities)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceCapabilities)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformationVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_get_SdpRecord(This, value) \
    ((This)->lpVtbl->get_SdpRecord(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_put_SdpRecord(This, value) \
    ((This)->lpVtbl->put_SdpRecord(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_get_LocalServiceId(This, value) \
    ((This)->lpVtbl->get_LocalServiceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_put_LocalServiceId(This, value) \
    ((This)->lpVtbl->put_LocalServiceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_get_ServiceCapabilities(This, value) \
    ((This)->lpVtbl->get_ServiceCapabilities(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_put_ServiceCapabilities(This, value) \
    ((This)->lpVtbl->put_ServiceCapabilities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommInboundConnectionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Background_IRfcommOutboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId** value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteServiceId)(__x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformationVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_get_RemoteServiceId(This, value) \
    ((This)->lpVtbl->get_RemoteServiceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_put_RemoteServiceId(This, value) \
    ((This)->lpVtbl->put_RemoteServiceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CBackground_CIRfcommOutboundConnectionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementPublisherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_BluetoothLEAdvertisementWatcherTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattCharacteristicNotificationTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattServiceProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection ** Default Interface **
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattServiceProviderConnection[] = L"Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_GattServiceProviderTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommConnectionTriggerDetails[] = L"Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommInboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Background_RfcommOutboundConnectionInformation[] = L"Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
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
#endif // __windows2Edevices2Ebluetooth2Ebackground_p_h__

#endif // __windows2Edevices2Ebluetooth2Ebackground_h__
