
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
#ifndef __windows2Edevices2Eusb_h__
#define __windows2Edevices2Eusb_h__
#ifndef __windows2Edevices2Eusb_p_h__
#define __windows2Edevices2Eusb_p_h__


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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbBulkInEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbBulkInPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe ABI::Windows::Devices::Usb::IUsbBulkInPipe

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbBulkOutEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbBulkOutPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe ABI::Windows::Devices::Usb::IUsbBulkOutPipe

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbConfiguration;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration ABI::Windows::Devices::Usb::IUsbConfiguration

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbConfigurationDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor ABI::Windows::Devices::Usb::IUsbConfigurationDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbConfigurationDescriptorStatics;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics ABI::Windows::Devices::Usb::IUsbConfigurationDescriptorStatics

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbControlRequestType;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType ABI::Windows::Devices::Usb::IUsbControlRequestType

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor ABI::Windows::Devices::Usb::IUsbDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDevice;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice ABI::Windows::Devices::Usb::IUsbDevice

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDeviceClass;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass ABI::Windows::Devices::Usb::IUsbDeviceClass

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDeviceClasses;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses ABI::Windows::Devices::Usb::IUsbDeviceClasses

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDeviceClassesStatics;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics ABI::Windows::Devices::Usb::IUsbDeviceClassesStatics

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDeviceDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor ABI::Windows::Devices::Usb::IUsbDeviceDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbDeviceStatics;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics ABI::Windows::Devices::Usb::IUsbDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor ABI::Windows::Devices::Usb::IUsbEndpointDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbEndpointDescriptorStatics;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics ABI::Windows::Devices::Usb::IUsbEndpointDescriptorStatics

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterface;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface ABI::Windows::Devices::Usb::IUsbInterface

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterfaceDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor ABI::Windows::Devices::Usb::IUsbInterfaceDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterfaceDescriptorStatics;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics ABI::Windows::Devices::Usb::IUsbInterfaceDescriptorStatics

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterfaceSetting;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting ABI::Windows::Devices::Usb::IUsbInterfaceSetting

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterruptInEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterruptInEventArgs;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs ABI::Windows::Devices::Usb::IUsbInterruptInEventArgs

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterruptInPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe ABI::Windows::Devices::Usb::IUsbInterruptInPipe

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterruptOutEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbInterruptOutPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe ABI::Windows::Devices::Usb::IUsbInterruptOutPipe

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbSetupPacket;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket ABI::Windows::Devices::Usb::IUsbSetupPacket

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                interface IUsbSetupPacketFactory;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory ABI::Windows::Devices::Usb::IUsbSetupPacketFactory

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbDevice;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2138c5ed-b71a-5166-9948-d55792748f5c"))
IAsyncOperation<ABI::Windows::Devices::Usb::UsbDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbDevice*, ABI::Windows::Devices::Usb::IUsbDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Usb.UsbDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Usb::UsbDevice*> __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7331254f-6caf-587d-9c2a-018c66d312db"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Usb::UsbDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbDevice*, ABI::Windows::Devices::Usb::IUsbDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Usb.UsbDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Usb::UsbDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bee8834-b9a7-5a80-a746-5ef097227878"))
IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c3d2fd-b8a1-5620-b746-7ee6d533aca3"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbBulkInEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea511030-89c4-503d-8caf-667f4230d2a9"))
IIterator<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbBulkInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("101b1fd9-f1c9-5dda-9ad4-71176fa839b2"))
IIterable<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbBulkInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbBulkInPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7af2c5b-528d-5cbb-a997-d830ade704c7"))
IIterator<ABI::Windows::Devices::Usb::UsbBulkInPipe*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInPipe*, ABI::Windows::Devices::Usb::IUsbBulkInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbBulkInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbBulkInPipe*> __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2201a671-42d2-508d-a848-64b5447083c8"))
IIterable<ABI::Windows::Devices::Usb::UsbBulkInPipe*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInPipe*, ABI::Windows::Devices::Usb::IUsbBulkInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbBulkInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbBulkInPipe*> __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbBulkOutEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a8b89ab3-883d-5361-9903-f489cc62bea5"))
IIterator<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbBulkOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b80beb39-62b3-5f59-b3e7-882cc9c5b0c0"))
IIterable<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbBulkOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbBulkOutPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("46dd2f6a-573b-5c45-b168-9223038491dd"))
IIterator<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutPipe*, ABI::Windows::Devices::Usb::IUsbBulkOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbBulkOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9824caba-5ca6-5c2d-80cf-1949026d7857"))
IIterable<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutPipe*, ABI::Windows::Devices::Usb::IUsbBulkOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbBulkOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("521598ed-0167-528e-990d-52abb712f072"))
IIterator<ABI::Windows::Devices::Usb::UsbDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbDescriptor*, ABI::Windows::Devices::Usb::IUsbDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbDescriptor*> __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("989909a5-5a03-51fb-bd94-84da7bda8819"))
IIterable<ABI::Windows::Devices::Usb::UsbDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbDescriptor*, ABI::Windows::Devices::Usb::IUsbDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbDescriptor*> __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterface;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("216b5a5f-63e3-5a9b-9c99-b09cbc0ff3b1"))
IIterator<ABI::Windows::Devices::Usb::UsbInterface*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterface*, ABI::Windows::Devices::Usb::IUsbInterface*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterface*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f54037ed-92e9-590d-b904-3ad7bfa9a621"))
IIterable<ABI::Windows::Devices::Usb::UsbInterface*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterface*, ABI::Windows::Devices::Usb::IUsbInterface*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterface*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterfaceSetting;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("71267ec7-5697-5dea-b2f8-14cf698ec0ad"))
IIterator<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterfaceSetting*, ABI::Windows::Devices::Usb::IUsbInterfaceSetting*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterfaceSetting>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1aaf5739-9c2c-533e-a0e9-d53fdb45d15d"))
IIterable<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterfaceSetting*, ABI::Windows::Devices::Usb::IUsbInterfaceSetting*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterfaceSetting>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterruptInEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6717500f-ec1c-5b12-bf33-0e3e3d244587"))
IIterator<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterruptInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8a7bac69-1f10-59c7-9837-72cfed7154a4"))
IIterable<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterruptInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterruptInPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e3a7b1c0-74f6-5292-a22a-672aa2b49985"))
IIterator<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::IUsbInterruptInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterruptInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("39aef336-18aa-5be4-86d9-e332fe2632f3"))
IIterable<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::IUsbInterruptInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterruptInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterruptOutEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b6426db-db32-5b51-adad-04532ea94acd"))
IIterator<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09393d62-2316-536b-8a10-7038884ab2a7"))
IIterable<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterruptOutPipe;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#define DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cbd8d8a8-2286-5cbd-a6e4-962742ffd91a"))
IIterator<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*, ABI::Windows::Devices::Usb::IUsbInterruptOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Usb.UsbInterruptOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t;
#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#define DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e61a011e-4abe-53f2-83b3-ed4a949d2e3f"))
IIterable<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*, ABI::Windows::Devices::Usb::IUsbInterruptOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Usb.UsbInterruptOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t;
#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9c69ac78-309e-5763-af26-9706ffa47ec0"))
IVectorView<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbBulkInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbBulkInEndpointDescriptor*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a93c84bc-6484-5959-b61a-703cc7115f6f"))
IVectorView<ABI::Windows::Devices::Usb::UsbBulkInPipe*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkInPipe*, ABI::Windows::Devices::Usb::IUsbBulkInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbBulkInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbBulkInPipe*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("22a53676-a3ea-5dcd-bb39-b28a5327c4a3"))
IVectorView<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbBulkOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbBulkOutEndpointDescriptor*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0a873512-15f1-5e8e-a72a-045cfd7a5e83"))
IVectorView<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbBulkOutPipe*, ABI::Windows::Devices::Usb::IUsbBulkOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbBulkOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbBulkOutPipe*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5408baa2-291e-537a-b61f-137062f7ff7d"))
IVectorView<ABI::Windows::Devices::Usb::UsbDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbDescriptor*, ABI::Windows::Devices::Usb::IUsbDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbDescriptor*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9c69ec7f-2e42-58cd-a74a-f4974811134d"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterface*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterface*, ABI::Windows::Devices::Usb::IUsbInterface*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterface*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("71194af7-77c2-54d5-a116-287f0b7fd53f"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterfaceSetting*, ABI::Windows::Devices::Usb::IUsbInterfaceSetting*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterfaceSetting>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterfaceSetting*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3fc7f890-218e-5057-904d-6387c591cc93"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterruptInEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterruptInEndpointDescriptor*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("37469574-b4c5-5ba0-9616-894dd822ff5b"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::IUsbInterruptInPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterruptInPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterruptInPipe*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("984e7e15-c5ac-5140-a3c0-b583190085d7"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*, ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterruptOutEndpointDescriptor*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#define DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("748196c8-83bf-5ec3-8d28-a3112b3ee3cc"))
IVectorView<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*, ABI::Windows::Devices::Usb::IUsbInterruptOutPipe*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Usb.UsbInterruptOutPipe>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Usb::UsbInterruptOutPipe*> __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t;
#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_byte_USE
#define DEF___FIReference_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e5198cc8-2873-55f5-b0a1-84ff9e4aad62"))
IReference<::byte> : IReference_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<::byte> __FIReference_1_byte_t;
#define __FIReference_1_byte ABI::Windows::Foundation::__FIReference_1_byte_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_byte_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterruptInEventArgs;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e6db9449-f36a-50f2-926c-2afd85c49f01"))
ITypedEventHandler<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::UsbInterruptInEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::IUsbInterruptInPipe*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Usb::UsbInterruptInEventArgs*, ABI::Windows::Devices::Usb::IUsbInterruptInEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Usb.UsbInterruptInPipe, Windows.Devices.Usb.UsbInterruptInEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Usb::UsbInterruptInPipe*, ABI::Windows::Devices::Usb::UsbInterruptInEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbControlRecipient : int UsbControlRecipient;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbControlTransferType : int UsbControlTransferType;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbEndpointType : int UsbEndpointType;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbReadOptions : unsigned int UsbReadOptions;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbTransferDirection : int UsbTransferDirection;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                typedef enum UsbWriteOptions : unsigned int UsbWriteOptions;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbConfiguration;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbConfigurationDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbControlRequestType;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbDeviceClass;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbDeviceDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbEndpointDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbInterfaceDescriptor;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                class UsbSetupPacket;
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Usb.UsbControlRecipient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbControlRecipient : int
                {
                    UsbControlRecipient_Device = 0,
                    UsbControlRecipient_SpecifiedInterface = 1,
                    UsbControlRecipient_Endpoint = 2,
                    UsbControlRecipient_Other = 3,
                    UsbControlRecipient_DefaultInterface = 4,
                };
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbControlTransferType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbControlTransferType : int
                {
                    UsbControlTransferType_Standard = 0,
                    UsbControlTransferType_Class = 1,
                    UsbControlTransferType_Vendor = 2,
                };
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbEndpointType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbEndpointType : int
                {
                    UsbEndpointType_Control = 0,
                    UsbEndpointType_Isochronous = 1,
                    UsbEndpointType_Bulk = 2,
                    UsbEndpointType_Interrupt = 3,
                };
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbReadOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbReadOptions : unsigned int
                {
                    UsbReadOptions_None = 0,
                    UsbReadOptions_AutoClearStall = 0x1,
                    UsbReadOptions_OverrideAutomaticBufferManagement = 0x2,
                    UsbReadOptions_IgnoreShortPacket = 0x4,
                    UsbReadOptions_AllowPartialReads = 0x8,
                };

                DEFINE_ENUM_FLAG_OPERATORS(UsbReadOptions)
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbTransferDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbTransferDirection : int
                {
                    UsbTransferDirection_Out = 0,
                    UsbTransferDirection_In = 1,
                };
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbWriteOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                enum UsbWriteOptions : unsigned int
                {
                    UsbWriteOptions_None = 0,
                    UsbWriteOptions_AutoClearStall = 0x1,
                    UsbWriteOptions_ShortPacketTerminate = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(UsbWriteOptions)
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkInEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkInEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbBulkInEndpointDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("3c6e4846-06cf-42a9-9dc2-971c1b14b6e3")
                IUsbBulkInEndpointDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPacketSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pipe(
                        ABI::Windows::Devices::Usb::IUsbBulkInPipe** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbBulkInEndpointDescriptor = __uuidof(IUsbBulkInEndpointDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkInPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkInPipe[] = L"Windows.Devices.Usb.IUsbBulkInPipe";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("f01d2d3b-4548-4d50-b326-d82cdabe1220")
                IUsbBulkInPipe : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxTransferSizeBytes(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearStallAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReadOptions(
                        ABI::Windows::Devices::Usb::UsbReadOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReadOptions(
                        ABI::Windows::Devices::Usb::UsbReadOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushBuffer(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbBulkInPipe = __uuidof(IUsbBulkInPipe);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkOutEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkOutEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("2820847a-ffee-4f60-9be1-956cac3ecb65")
                IUsbBulkOutEndpointDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPacketSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pipe(
                        ABI::Windows::Devices::Usb::IUsbBulkOutPipe** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbBulkOutEndpointDescriptor = __uuidof(IUsbBulkOutEndpointDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkOutPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkOutPipe[] = L"Windows.Devices.Usb.IUsbBulkOutPipe";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("a8e9ee6e-0115-45aa-8b21-37b225bccee7")
                IUsbBulkOutPipe : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearStallAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WriteOptions(
                        ABI::Windows::Devices::Usb::UsbWriteOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WriteOptions(
                        ABI::Windows::Devices::Usb::UsbWriteOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbBulkOutPipe = __uuidof(IUsbBulkOutPipe);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfiguration[] = L"Windows.Devices.Usb.IUsbConfiguration";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("68177429-36a9-46d7-b873-fc689251ec30")
                IUsbConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UsbInterfaces(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConfigurationDescriptor(
                        ABI::Windows::Devices::Usb::IUsbConfigurationDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Descriptors(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbConfiguration = __uuidof(IUsbConfiguration);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfigurationDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfigurationDescriptor[] = L"Windows.Devices.Usb.IUsbConfigurationDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("f2176d92-b442-407a-8207-7d646c0385f3")
                IUsbConfigurationDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConfigurationValue(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPowerMilliamps(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelfPowered(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteWakeup(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbConfigurationDescriptor = __uuidof(IUsbConfigurationDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfigurationDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfigurationDescriptorStatics[] = L"Windows.Devices.Usb.IUsbConfigurationDescriptorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("424ced93-e740-40a1-92bd-da120ea04914")
                IUsbConfigurationDescriptorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryParse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbConfigurationDescriptor** parsed,
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Parse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbConfigurationDescriptor** parsed
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbConfigurationDescriptorStatics = __uuidof(IUsbConfigurationDescriptorStatics);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbControlRequestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbControlRequestType
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbControlRequestType[] = L"Windows.Devices.Usb.IUsbControlRequestType";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("8e9465a6-d73d-46de-94be-aae7f07c0f5c")
                IUsbControlRequestType : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Direction(
                        ABI::Windows::Devices::Usb::UsbTransferDirection* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Direction(
                        ABI::Windows::Devices::Usb::UsbTransferDirection value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControlTransferType(
                        ABI::Windows::Devices::Usb::UsbControlTransferType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ControlTransferType(
                        ABI::Windows::Devices::Usb::UsbControlTransferType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Recipient(
                        ABI::Windows::Devices::Usb::UsbControlRecipient* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Recipient(
                        ABI::Windows::Devices::Usb::UsbControlRecipient value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AsByte(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AsByte(
                        BYTE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbControlRequestType = __uuidof(IUsbControlRequestType);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDescriptor[] = L"Windows.Devices.Usb.IUsbDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("0a89f216-5f9d-4874-8904-da9ad3f5528f")
                IUsbDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Length(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DescriptorType(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadDescriptorBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDescriptor = __uuidof(IUsbDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDevice[] = L"Windows.Devices.Usb.IUsbDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("5249b992-c456-44d5-ad5e-24f5a089f63b")
                IUsbDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SendControlOutTransferAsync(
                        ABI::Windows::Devices::Usb::IUsbSetupPacket* setupPacket,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendControlOutTransferAsyncNoBuffer(
                        ABI::Windows::Devices::Usb::IUsbSetupPacket* setupPacket,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendControlInTransferAsync(
                        ABI::Windows::Devices::Usb::IUsbSetupPacket* setupPacket,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendControlInTransferAsyncNoBuffer(
                        ABI::Windows::Devices::Usb::IUsbSetupPacket* setupPacket,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultInterface(
                        ABI::Windows::Devices::Usb::IUsbInterface** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceDescriptor(
                        ABI::Windows::Devices::Usb::IUsbDeviceDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::Devices::Usb::IUsbConfiguration** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDevice = __uuidof(IUsbDevice);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClass
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClass[] = L"Windows.Devices.Usb.IUsbDeviceClass";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("051942f9-845e-47eb-b12a-38f2f617afe7")
                IUsbDeviceClass : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ClassCode(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClassCode(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubclassCode(
                        __FIReference_1_byte** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SubclassCode(
                        __FIReference_1_byte* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolCode(
                        __FIReference_1_byte** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProtocolCode(
                        __FIReference_1_byte* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDeviceClass = __uuidof(IUsbDeviceClass);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClasses
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClasses
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClasses[] = L"Windows.Devices.Usb.IUsbDeviceClasses";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("686f955d-9b92-4b30-9781-c22c55ac35cb")
                IUsbDeviceClasses : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IUsbDeviceClasses = __uuidof(IUsbDeviceClasses);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClassesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClasses
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClassesStatics[] = L"Windows.Devices.Usb.IUsbDeviceClassesStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("b20b0527-c580-4599-a165-981b4fd03230")
                IUsbDeviceClassesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CdcControl(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Physical(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PersonalHealthcare(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActiveSync(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PalmSync(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceFirmwareUpdate(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Irda(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Measurement(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VendorSpecific(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDeviceClassesStatics = __uuidof(IUsbDeviceClassesStatics);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceDescriptor[] = L"Windows.Devices.Usb.IUsbDeviceDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("1f48d1f6-ba97-4322-b92c-b5b189216588")
                IUsbDeviceDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BcdUsb(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPacketSize0(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VendorId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BcdDeviceRevision(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumberOfConfigurations(
                        BYTE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDeviceDescriptor = __uuidof(IUsbDeviceDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceStatics[] = L"Windows.Devices.Usb.IUsbDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("066b85a2-09b7-4446-8502-6fe6dcaa7309")
                IUsbDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        UINT32 vendorId,
                        UINT32 productId,
                        GUID winUsbInterfaceClass,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorGuidOnly(
                        GUID winUsbInterfaceClass,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorVidPidOnly(
                        UINT32 vendorId,
                        UINT32 productId,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceClassSelector(
                        ABI::Windows::Devices::Usb::IUsbDeviceClass* usbClass,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbDeviceStatics = __uuidof(IUsbDeviceStatics);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbEndpointDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("6b4862d9-8df7-4b40-ac83-578f139f0575")
                IUsbEndpointDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Direction(
                        ABI::Windows::Devices::Usb::UsbTransferDirection* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointType(
                        ABI::Windows::Devices::Usb::UsbEndpointType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AsBulkInEndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbBulkInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AsInterruptInEndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AsBulkOutEndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbBulkOutEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AsInterruptOutEndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbEndpointDescriptor = __uuidof(IUsbEndpointDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbEndpointDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbEndpointDescriptorStatics[] = L"Windows.Devices.Usb.IUsbEndpointDescriptorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("c890b201-9a6a-495e-a82c-295b9e708106")
                IUsbEndpointDescriptorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryParse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbEndpointDescriptor** parsed,
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Parse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbEndpointDescriptor** parsed
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbEndpointDescriptorStatics = __uuidof(IUsbEndpointDescriptorStatics);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterface
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterface[] = L"Windows.Devices.Usb.IUsbInterface";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("a0322b95-7f47-48ab-a727-678c25be2112")
                IUsbInterface : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BulkInPipes(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterruptInPipes(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BulkOutPipes(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterruptOutPipes(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterfaceSettings(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterfaceNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Descriptors(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterface = __uuidof(IUsbInterface);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterface;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceDescriptor[] = L"Windows.Devices.Usb.IUsbInterfaceDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("199670c7-b7ee-4f90-8cd5-94a2e257598a")
                IUsbInterfaceDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ClassCode(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubclassCode(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolCode(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateSettingNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterfaceNumber(
                        BYTE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterfaceDescriptor = __uuidof(IUsbInterfaceDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceDescriptorStatics[] = L"Windows.Devices.Usb.IUsbInterfaceDescriptorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("e34a9ff5-77d6-48b6-b0be-16c6422316fe")
                IUsbInterfaceDescriptorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryParse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbInterfaceDescriptor** parsed,
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Parse(
                        ABI::Windows::Devices::Usb::IUsbDescriptor* descriptor,
                        ABI::Windows::Devices::Usb::IUsbInterfaceDescriptor** parsed
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterfaceDescriptorStatics = __uuidof(IUsbInterfaceDescriptorStatics);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceSetting
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceSetting[] = L"Windows.Devices.Usb.IUsbInterfaceSetting";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("1827bba7-8da7-4af7-8f4c-7f3032e781f5")
                IUsbInterfaceSetting : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BulkInEndpoints(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterruptInEndpoints(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BulkOutEndpoints(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterruptOutEndpoints(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Selected(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectSettingAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterfaceDescriptor(
                        ABI::Windows::Devices::Usb::IUsbInterfaceDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Descriptors(
                        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterfaceSetting = __uuidof(IUsbInterfaceSetting);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("c0528967-c911-4c3a-86b2-419c2da89039")
                IUsbInterruptInEndpointDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPacketSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Interval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pipe(
                        ABI::Windows::Devices::Usb::IUsbInterruptInPipe** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterruptInEndpointDescriptor = __uuidof(IUsbInterruptInEndpointDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInEventArgs[] = L"Windows.Devices.Usb.IUsbInterruptInEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("b7b04092-1418-4936-8209-299cf5605583")
                IUsbInterruptInEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InterruptData(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterruptInEventArgs = __uuidof(IUsbInterruptInEventArgs);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInPipe[] = L"Windows.Devices.Usb.IUsbInterruptInPipe";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("fa007116-84d7-48c7-8a3f-4c0b235f2ea6")
                IUsbInterruptInPipe : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbInterruptInEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearStallAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DataReceived(
                        __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DataReceived(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterruptInPipe = __uuidof(IUsbInterruptInPipe);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptOutEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("cc9fed81-10ca-4533-952d-9e278341e80f")
                IUsbInterruptOutEndpointDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPacketSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointNumber(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Interval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pipe(
                        ABI::Windows::Devices::Usb::IUsbInterruptOutPipe** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterruptOutEndpointDescriptor = __uuidof(IUsbInterruptOutEndpointDescriptor);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptOutPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptOutPipe[] = L"Windows.Devices.Usb.IUsbInterruptOutPipe";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("e984c8a9-aaf9-49d0-b96c-f661ab4a7f95")
                IUsbInterruptOutPipe : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EndpointDescriptor(
                        ABI::Windows::Devices::Usb::IUsbInterruptOutEndpointDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearStallAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WriteOptions(
                        ABI::Windows::Devices::Usb::UsbWriteOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WriteOptions(
                        ABI::Windows::Devices::Usb::UsbWriteOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbInterruptOutPipe = __uuidof(IUsbInterruptOutPipe);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbSetupPacket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbSetupPacket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbSetupPacket[] = L"Windows.Devices.Usb.IUsbSetupPacket";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("104ba132-c78f-4c51-b654-e49d02f2cb03")
                IUsbSetupPacket : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequestType(
                        ABI::Windows::Devices::Usb::IUsbControlRequestType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestType(
                        ABI::Windows::Devices::Usb::IUsbControlRequestType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Request(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Index(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Index(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Length(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Length(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbSetupPacket = __uuidof(IUsbSetupPacket);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbSetupPacketFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbSetupPacket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbSetupPacketFactory[] = L"Windows.Devices.Usb.IUsbSetupPacketFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Usb {
                MIDL_INTERFACE("c9257d50-1b2e-4a41-a2a7-338f0cef3c14")
                IUsbSetupPacketFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithEightByteBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* eightByteBuffer,
                        ABI::Windows::Devices::Usb::IUsbSetupPacket** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUsbSetupPacketFactory = __uuidof(IUsbSetupPacketFactory);
            } /* Usb */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkInEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkInEndpointDescriptor[] = L"Windows.Devices.Usb.UsbBulkInEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkInPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkInPipe[] = L"Windows.Devices.Usb.UsbBulkInPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor[] = L"Windows.Devices.Usb.UsbBulkOutEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkOutPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkOutPipe[] = L"Windows.Devices.Usb.UsbBulkOutPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbConfiguration[] = L"Windows.Devices.Usb.UsbConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbConfigurationDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbConfigurationDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbConfigurationDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbConfigurationDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbConfigurationDescriptor[] = L"Windows.Devices.Usb.UsbConfigurationDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbControlRequestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbControlRequestType ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbControlRequestType_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbControlRequestType_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbControlRequestType[] = L"Windows.Devices.Usb.UsbControlRequestType";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDescriptor[] = L"Windows.Devices.Usb.UsbDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDevice[] = L"Windows.Devices.Usb.UsbDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceClass ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClass_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClass_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceClass[] = L"Windows.Devices.Usb.UsbDeviceClass";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceClasses
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbDeviceClassesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceClasses ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClasses_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClasses_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceClasses[] = L"Windows.Devices.Usb.UsbDeviceClasses";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceDescriptor[] = L"Windows.Devices.Usb.UsbDeviceDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbEndpointDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbEndpointDescriptor[] = L"Windows.Devices.Usb.UsbEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterface ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterface_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterface_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterface[] = L"Windows.Devices.Usb.UsbInterface";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbInterfaceDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterfaceDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterfaceDescriptor[] = L"Windows.Devices.Usb.UsbInterfaceDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterfaceSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterfaceSetting ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceSetting_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceSetting_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterfaceSetting[] = L"Windows.Devices.Usb.UsbInterfaceSetting";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor[] = L"Windows.Devices.Usb.UsbInterruptInEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInEventArgs[] = L"Windows.Devices.Usb.UsbInterruptInEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInPipe[] = L"Windows.Devices.Usb.UsbInterruptInPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor[] = L"Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptOutPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptOutPipe[] = L"Windows.Devices.Usb.UsbInterruptOutPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbSetupPacket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Usb.IUsbSetupPacketFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbSetupPacket ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbSetupPacket_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbSetupPacket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbSetupPacket[] = L"Windows.Devices.Usb.UsbSetupPacket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory;

#endif // ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CUsb__CUsbDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkInPipe** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbBulkOutPipe** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterface;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterface* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterface;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterface* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterface** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterfaceSetting** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptInPipe** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

typedef struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl;

interface __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

typedef struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        __FIIterator_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl;

interface __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSettingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe;

typedef struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl;

interface __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_byte_INTERFACE_DEFINED__)
#define ____FIReference_1_byte_INTERFACE_DEFINED__

typedef interface __FIReference_1_byte __FIReference_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_byte;

typedef struct __FIReference_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_byte* This,
        BYTE* result);

    END_INTERFACE
} __FIReference_1_byteVtbl;

interface __FIReference_1_byte
{
    CONST_VTBL struct __FIReference_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_byte_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_byte_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* sender,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlRecipient __x_ABI_CWindows_CDevices_CUsb_CUsbControlRecipient;

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlTransferType __x_ABI_CWindows_CDevices_CUsb_CUsbControlTransferType;

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbEndpointType __x_ABI_CWindows_CDevices_CUsb_CUsbEndpointType;

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbReadOptions __x_ABI_CWindows_CDevices_CUsb_CUsbReadOptions;

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection;

typedef enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions;

/*
 *
 * Struct Windows.Devices.Usb.UsbControlRecipient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlRecipient
{
    UsbControlRecipient_Device = 0,
    UsbControlRecipient_SpecifiedInterface = 1,
    UsbControlRecipient_Endpoint = 2,
    UsbControlRecipient_Other = 3,
    UsbControlRecipient_DefaultInterface = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbControlTransferType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlTransferType
{
    UsbControlTransferType_Standard = 0,
    UsbControlTransferType_Class = 1,
    UsbControlTransferType_Vendor = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbEndpointType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbEndpointType
{
    UsbEndpointType_Control = 0,
    UsbEndpointType_Isochronous = 1,
    UsbEndpointType_Bulk = 2,
    UsbEndpointType_Interrupt = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbReadOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbReadOptions
{
    UsbReadOptions_None = 0,
    UsbReadOptions_AutoClearStall = 0x1,
    UsbReadOptions_OverrideAutomaticBufferManagement = 0x2,
    UsbReadOptions_IgnoreShortPacket = 0x4,
    UsbReadOptions_AllowPartialReads = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbTransferDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection
{
    UsbTransferDirection_Out = 0,
    UsbTransferDirection_In = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Usb.UsbWriteOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions
{
    UsbWriteOptions_None = 0,
    UsbWriteOptions_AutoClearStall = 0x1,
    UsbWriteOptions_ShortPacketTerminate = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkInEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkInEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbBulkInEndpointDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPacketSize)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Pipe)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_get_MaxPacketSize(This, value) \
    ((This)->lpVtbl->get_MaxPacketSize(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_get_EndpointNumber(This, value) \
    ((This)->lpVtbl->get_EndpointNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_get_Pipe(This, value) \
    ((This)->lpVtbl->get_Pipe(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkInPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkInPipe[] = L"Windows.Devices.Usb.IUsbBulkInPipe";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxTransferSizeBytes)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* ClearStallAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* put_ReadOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbReadOptions value);
    HRESULT (STDMETHODCALLTYPE* get_ReadOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbReadOptions* value);
    HRESULT (STDMETHODCALLTYPE* FlushBuffer)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipeVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_get_MaxTransferSizeBytes(This, value) \
    ((This)->lpVtbl->get_MaxTransferSizeBytes(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_get_EndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_EndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_ClearStallAsync(This, operation) \
    ((This)->lpVtbl->ClearStallAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_put_ReadOptions(This, value) \
    ((This)->lpVtbl->put_ReadOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_get_ReadOptions(This, value) \
    ((This)->lpVtbl->get_ReadOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_FlushBuffer(This) \
    ((This)->lpVtbl->FlushBuffer(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkOutEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkOutEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPacketSize)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Pipe)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_get_MaxPacketSize(This, value) \
    ((This)->lpVtbl->get_MaxPacketSize(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_get_EndpointNumber(This, value) \
    ((This)->lpVtbl->get_EndpointNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_get_Pipe(This, value) \
    ((This)->lpVtbl->get_Pipe(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbBulkOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbBulkOutPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbBulkOutPipe[] = L"Windows.Devices.Usb.IUsbBulkOutPipe";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* ClearStallAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* put_WriteOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions value);
    HRESULT (STDMETHODCALLTYPE* get_WriteOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions* value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipeVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_get_EndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_EndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_ClearStallAsync(This, operation) \
    ((This)->lpVtbl->ClearStallAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_put_WriteOptions(This, value) \
    ((This)->lpVtbl->put_WriteOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_get_WriteOptions(This, value) \
    ((This)->lpVtbl->get_WriteOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfiguration[] = L"Windows.Devices.Usb.IUsbConfiguration";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UsbInterfaces)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterface** value);
    HRESULT (STDMETHODCALLTYPE* get_ConfigurationDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_Descriptors)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_get_UsbInterfaces(This, value) \
    ((This)->lpVtbl->get_UsbInterfaces(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_get_ConfigurationDescriptor(This, value) \
    ((This)->lpVtbl->get_ConfigurationDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_get_Descriptors(This, value) \
    ((This)->lpVtbl->get_Descriptors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfigurationDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfigurationDescriptor[] = L"Windows.Devices.Usb.IUsbConfigurationDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConfigurationValue)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPowerMilliamps)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelfPowered)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteWakeup)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_get_ConfigurationValue(This, value) \
    ((This)->lpVtbl->get_ConfigurationValue(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_get_MaxPowerMilliamps(This, value) \
    ((This)->lpVtbl->get_MaxPowerMilliamps(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_get_SelfPowered(This, value) \
    ((This)->lpVtbl->get_SelfPowered(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_get_RemoteWakeup(This, value) \
    ((This)->lpVtbl->get_RemoteWakeup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbConfigurationDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbConfigurationDescriptorStatics[] = L"Windows.Devices.Usb.IUsbConfigurationDescriptorStatics";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryParse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor** parsed,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* Parse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptor** parsed);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_TryParse(This, descriptor, parsed, success) \
    ((This)->lpVtbl->TryParse(This, descriptor, parsed, success))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_Parse(This, descriptor, parsed) \
    ((This)->lpVtbl->Parse(This, descriptor, parsed))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbConfigurationDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbControlRequestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbControlRequestType
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbControlRequestType[] = L"Windows.Devices.Usb.IUsbControlRequestType";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Direction)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection* value);
    HRESULT (STDMETHODCALLTYPE* put_Direction)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection value);
    HRESULT (STDMETHODCALLTYPE* get_ControlTransferType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlTransferType* value);
    HRESULT (STDMETHODCALLTYPE* put_ControlTransferType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlTransferType value);
    HRESULT (STDMETHODCALLTYPE* get_Recipient)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlRecipient* value);
    HRESULT (STDMETHODCALLTYPE* put_Recipient)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbControlRecipient value);
    HRESULT (STDMETHODCALLTYPE* get_AsByte)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_AsByte)(__x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* This,
        BYTE value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestTypeVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_get_Direction(This, value) \
    ((This)->lpVtbl->get_Direction(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_put_Direction(This, value) \
    ((This)->lpVtbl->put_Direction(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_get_ControlTransferType(This, value) \
    ((This)->lpVtbl->get_ControlTransferType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_put_ControlTransferType(This, value) \
    ((This)->lpVtbl->put_ControlTransferType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_get_Recipient(This, value) \
    ((This)->lpVtbl->get_Recipient(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_put_Recipient(This, value) \
    ((This)->lpVtbl->put_Recipient(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_get_AsByte(This, value) \
    ((This)->lpVtbl->get_AsByte(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_put_AsByte(This, value) \
    ((This)->lpVtbl->put_AsByte(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDescriptor[] = L"Windows.Devices.Usb.IUsbDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_DescriptorType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* ReadDescriptorBuffer)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_get_DescriptorType(This, value) \
    ((This)->lpVtbl->get_DescriptorType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_ReadDescriptorBuffer(This, buffer) \
    ((This)->lpVtbl->ReadDescriptorBuffer(This, buffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDevice[] = L"Windows.Devices.Usb.IUsbDevice";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendControlOutTransferAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* setupPacket,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* SendControlOutTransferAsyncNoBuffer)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* setupPacket,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* SendControlInTransferAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* setupPacket,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);
    HRESULT (STDMETHODCALLTYPE* SendControlInTransferAsyncNoBuffer)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* setupPacket,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);
    HRESULT (STDMETHODCALLTYPE* get_DefaultInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDevice* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_SendControlOutTransferAsync(This, setupPacket, buffer, operation) \
    ((This)->lpVtbl->SendControlOutTransferAsync(This, setupPacket, buffer, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_SendControlOutTransferAsyncNoBuffer(This, setupPacket, operation) \
    ((This)->lpVtbl->SendControlOutTransferAsyncNoBuffer(This, setupPacket, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_SendControlInTransferAsync(This, setupPacket, buffer, operation) \
    ((This)->lpVtbl->SendControlInTransferAsync(This, setupPacket, buffer, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_SendControlInTransferAsyncNoBuffer(This, setupPacket, operation) \
    ((This)->lpVtbl->SendControlInTransferAsyncNoBuffer(This, setupPacket, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_get_DefaultInterface(This, value) \
    ((This)->lpVtbl->get_DefaultInterface(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_get_DeviceDescriptor(This, value) \
    ((This)->lpVtbl->get_DeviceDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_get_Configuration(This, value) \
    ((This)->lpVtbl->get_Configuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClass
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClass[] = L"Windows.Devices.Usb.IUsbDeviceClass";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_ClassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_SubclassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        __FIReference_1_byte** value);
    HRESULT (STDMETHODCALLTYPE* put_SubclassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        __FIReference_1_byte* value);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        __FIReference_1_byte** value);
    HRESULT (STDMETHODCALLTYPE* put_ProtocolCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* This,
        __FIReference_1_byte* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_get_ClassCode(This, value) \
    ((This)->lpVtbl->get_ClassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_put_ClassCode(This, value) \
    ((This)->lpVtbl->put_ClassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_get_SubclassCode(This, value) \
    ((This)->lpVtbl->get_SubclassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_put_SubclassCode(This, value) \
    ((This)->lpVtbl->put_SubclassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_get_ProtocolCode(This, value) \
    ((This)->lpVtbl->get_ProtocolCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_put_ProtocolCode(This, value) \
    ((This)->lpVtbl->put_ProtocolCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClasses
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClasses
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClasses[] = L"Windows.Devices.Usb.IUsbDeviceClasses";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClasses_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceClassesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceClasses
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceClassesStatics[] = L"Windows.Devices.Usb.IUsbDeviceClassesStatics";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CdcControl)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Physical)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_PersonalHealthcare)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ActiveSync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_PalmSync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceFirmwareUpdate)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Irda)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Measurement)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_VendorSpecific)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_CdcControl(This, value) \
    ((This)->lpVtbl->get_CdcControl(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_Physical(This, value) \
    ((This)->lpVtbl->get_Physical(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_PersonalHealthcare(This, value) \
    ((This)->lpVtbl->get_PersonalHealthcare(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_ActiveSync(This, value) \
    ((This)->lpVtbl->get_ActiveSync(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_PalmSync(This, value) \
    ((This)->lpVtbl->get_PalmSync(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_DeviceFirmwareUpdate(This, value) \
    ((This)->lpVtbl->get_DeviceFirmwareUpdate(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_Irda(This, value) \
    ((This)->lpVtbl->get_Irda(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_Measurement(This, value) \
    ((This)->lpVtbl->get_Measurement(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_get_VendorSpecific(This, value) \
    ((This)->lpVtbl->get_VendorSpecific(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClassesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDeviceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceDescriptor[] = L"Windows.Devices.Usb.IUsbDeviceDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BcdUsb)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPacketSize0)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_VendorId)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_BcdDeviceRevision)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfConfigurations)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor* This,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_BcdUsb(This, value) \
    ((This)->lpVtbl->get_BcdUsb(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_MaxPacketSize0(This, value) \
    ((This)->lpVtbl->get_MaxPacketSize0(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_VendorId(This, value) \
    ((This)->lpVtbl->get_VendorId(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_BcdDeviceRevision(This, value) \
    ((This)->lpVtbl->get_BcdDeviceRevision(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_get_NumberOfConfigurations(This, value) \
    ((This)->lpVtbl->get_NumberOfConfigurations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbDeviceStatics[] = L"Windows.Devices.Usb.IUsbDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        UINT32 vendorId,
        UINT32 productId,
        GUID winUsbInterfaceClass,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorGuidOnly)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        GUID winUsbInterfaceClass,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorVidPidOnly)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        UINT32 vendorId,
        UINT32 productId,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceClassSelector)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceClass* usbClass,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CUsb__CUsbDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetDeviceSelector(This, vendorId, productId, winUsbInterfaceClass, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, vendorId, productId, winUsbInterfaceClass, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetDeviceSelectorGuidOnly(This, winUsbInterfaceClass, value) \
    ((This)->lpVtbl->GetDeviceSelectorGuidOnly(This, winUsbInterfaceClass, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetDeviceSelectorVidPidOnly(This, vendorId, productId, value) \
    ((This)->lpVtbl->GetDeviceSelectorVidPidOnly(This, vendorId, productId, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_GetDeviceClassSelector(This, usbClass, value) \
    ((This)->lpVtbl->GetDeviceClassSelector(This, usbClass, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbEndpointDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Direction)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbTransferDirection* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbEndpointType* value);
    HRESULT (STDMETHODCALLTYPE* get_AsBulkInEndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_AsInterruptInEndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_AsBulkOutEndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbBulkOutEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_AsInterruptOutEndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_EndpointNumber(This, value) \
    ((This)->lpVtbl->get_EndpointNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_Direction(This, value) \
    ((This)->lpVtbl->get_Direction(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_EndpointType(This, value) \
    ((This)->lpVtbl->get_EndpointType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_AsBulkInEndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_AsBulkInEndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_AsInterruptInEndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_AsInterruptInEndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_AsBulkOutEndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_AsBulkOutEndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_get_AsInterruptOutEndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_AsInterruptOutEndpointDescriptor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbEndpointDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbEndpointDescriptorStatics[] = L"Windows.Devices.Usb.IUsbEndpointDescriptorStatics";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryParse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor** parsed,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* Parse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptor** parsed);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_TryParse(This, descriptor, parsed, success) \
    ((This)->lpVtbl->TryParse(This, descriptor, parsed, success))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_Parse(This, descriptor, parsed) \
    ((This)->lpVtbl->Parse(This, descriptor, parsed))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbEndpointDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterface
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterface[] = L"Windows.Devices.Usb.IUsbInterface";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BulkInPipes)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInPipe** value);
    HRESULT (STDMETHODCALLTYPE* get_InterruptInPipes)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInPipe** value);
    HRESULT (STDMETHODCALLTYPE* get_BulkOutPipes)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutPipe** value);
    HRESULT (STDMETHODCALLTYPE* get_InterruptOutPipes)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutPipe** value);
    HRESULT (STDMETHODCALLTYPE* get_InterfaceSettings)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterfaceSetting** value);
    HRESULT (STDMETHODCALLTYPE* get_InterfaceNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Descriptors)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterface* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_BulkInPipes(This, value) \
    ((This)->lpVtbl->get_BulkInPipes(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_InterruptInPipes(This, value) \
    ((This)->lpVtbl->get_InterruptInPipes(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_BulkOutPipes(This, value) \
    ((This)->lpVtbl->get_BulkOutPipes(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_InterruptOutPipes(This, value) \
    ((This)->lpVtbl->get_InterruptOutPipes(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_InterfaceSettings(This, value) \
    ((This)->lpVtbl->get_InterfaceSettings(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_InterfaceNumber(This, value) \
    ((This)->lpVtbl->get_InterfaceNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_get_Descriptors(This, value) \
    ((This)->lpVtbl->get_Descriptors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterface;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceDescriptor[] = L"Windows.Devices.Usb.IUsbInterfaceDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_SubclassCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolCode)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_AlternateSettingNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_InterfaceNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor* This,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_get_ClassCode(This, value) \
    ((This)->lpVtbl->get_ClassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_get_SubclassCode(This, value) \
    ((This)->lpVtbl->get_SubclassCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_get_ProtocolCode(This, value) \
    ((This)->lpVtbl->get_ProtocolCode(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_get_AlternateSettingNumber(This, value) \
    ((This)->lpVtbl->get_AlternateSettingNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_get_InterfaceNumber(This, value) \
    ((This)->lpVtbl->get_InterfaceNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceDescriptorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceDescriptorStatics[] = L"Windows.Devices.Usb.IUsbInterfaceDescriptorStatics";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryParse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor** parsed,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* Parse)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbDescriptor* descriptor,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor** parsed);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_TryParse(This, descriptor, parsed, success) \
    ((This)->lpVtbl->TryParse(This, descriptor, parsed, success))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_Parse(This, descriptor, parsed) \
    ((This)->lpVtbl->Parse(This, descriptor, parsed))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterfaceSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterfaceSetting
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterfaceSetting[] = L"Windows.Devices.Usb.IUsbInterfaceSetting";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSettingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BulkInEndpoints)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_InterruptInEndpoints)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_BulkOutEndpoints)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbBulkOutEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_InterruptOutEndpoints)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbInterruptOutEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_Selected)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SelectSettingAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_InterfaceDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_Descriptors)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting* This,
        __FIVectorView_1_Windows__CDevices__CUsb__CUsbDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSettingVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSettingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_BulkInEndpoints(This, value) \
    ((This)->lpVtbl->get_BulkInEndpoints(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_InterruptInEndpoints(This, value) \
    ((This)->lpVtbl->get_InterruptInEndpoints(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_BulkOutEndpoints(This, value) \
    ((This)->lpVtbl->get_BulkOutEndpoints(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_InterruptOutEndpoints(This, value) \
    ((This)->lpVtbl->get_InterruptOutEndpoints(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_Selected(This, value) \
    ((This)->lpVtbl->get_Selected(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_SelectSettingAsync(This, operation) \
    ((This)->lpVtbl->SelectSettingAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_InterfaceDescriptor(This, value) \
    ((This)->lpVtbl->get_InterfaceDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_get_Descriptors(This, value) \
    ((This)->lpVtbl->get_Descriptors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterfaceSetting_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPacketSize)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Interval)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Pipe)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_get_MaxPacketSize(This, value) \
    ((This)->lpVtbl->get_MaxPacketSize(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_get_EndpointNumber(This, value) \
    ((This)->lpVtbl->get_EndpointNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_get_Interval(This, value) \
    ((This)->lpVtbl->get_Interval(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_get_Pipe(This, value) \
    ((This)->lpVtbl->get_Pipe(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInEventArgs[] = L"Windows.Devices.Usb.IUsbInterruptInEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InterruptData)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_get_InterruptData(This, value) \
    ((This)->lpVtbl->get_InterruptData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptInPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptInPipe[] = L"Windows.Devices.Usb.IUsbInterruptInPipe";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* ClearStallAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_DataReceived)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        __FITypedEventHandler_2_Windows__CDevices__CUsb__CUsbInterruptInPipe_Windows__CDevices__CUsb__CUsbInterruptInEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DataReceived)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipeVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_get_EndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_EndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_ClearStallAsync(This, operation) \
    ((This)->lpVtbl->ClearStallAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_add_DataReceived(This, handler, token) \
    ((This)->lpVtbl->add_DataReceived(This, handler, token))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_remove_DataReceived(This, token) \
    ((This)->lpVtbl->remove_DataReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptInPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptOutEndpointDescriptor[] = L"Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPacketSize)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_EndpointNumber)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Interval)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Pipe)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptorVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_get_MaxPacketSize(This, value) \
    ((This)->lpVtbl->get_MaxPacketSize(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_get_EndpointNumber(This, value) \
    ((This)->lpVtbl->get_EndpointNumber(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_get_Interval(This, value) \
    ((This)->lpVtbl->get_Interval(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_get_Pipe(This, value) \
    ((This)->lpVtbl->get_Pipe(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbInterruptOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbInterruptOutPipe
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbInterruptOutPipe[] = L"Windows.Devices.Usb.IUsbInterruptOutPipe";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EndpointDescriptor)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutEndpointDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* ClearStallAsync)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* put_WriteOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions value);
    HRESULT (STDMETHODCALLTYPE* get_WriteOptions)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        enum __x_ABI_CWindows_CDevices_CUsb_CUsbWriteOptions* value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipeVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_get_EndpointDescriptor(This, value) \
    ((This)->lpVtbl->get_EndpointDescriptor(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_ClearStallAsync(This, operation) \
    ((This)->lpVtbl->ClearStallAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_put_WriteOptions(This, value) \
    ((This)->lpVtbl->put_WriteOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_get_WriteOptions(This, value) \
    ((This)->lpVtbl->get_WriteOptions(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbInterruptOutPipe_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbSetupPacket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbSetupPacket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbSetupPacket[] = L"Windows.Devices.Usb.IUsbSetupPacket";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType** value);
    HRESULT (STDMETHODCALLTYPE* put_RequestType)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbControlRequestType* value);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_Request)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Index)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Index)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Length)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_get_RequestType(This, value) \
    ((This)->lpVtbl->get_RequestType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_put_RequestType(This, value) \
    ((This)->lpVtbl->put_RequestType(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_put_Request(This, value) \
    ((This)->lpVtbl->put_Request(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_get_Index(This, value) \
    ((This)->lpVtbl->get_Index(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_put_Index(This, value) \
    ((This)->lpVtbl->put_Index(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_put_Length(This, value) \
    ((This)->lpVtbl->put_Length(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Usb.IUsbSetupPacketFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Usb.UsbSetupPacket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Usb_IUsbSetupPacketFactory[] = L"Windows.Devices.Usb.IUsbSetupPacketFactory";
typedef struct __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithEightByteBuffer)(__x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* eightByteBuffer,
        __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacket** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_CreateWithEightByteBuffer(This, eightByteBuffer, value) \
    ((This)->lpVtbl->CreateWithEightByteBuffer(This, eightByteBuffer, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CUsb_CIUsbSetupPacketFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkInEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkInEndpointDescriptor[] = L"Windows.Devices.Usb.UsbBulkInEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkInPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkInPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkInPipe[] = L"Windows.Devices.Usb.UsbBulkInPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkOutEndpointDescriptor[] = L"Windows.Devices.Usb.UsbBulkOutEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbBulkOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbBulkOutPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbBulkOutPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbBulkOutPipe[] = L"Windows.Devices.Usb.UsbBulkOutPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbConfiguration[] = L"Windows.Devices.Usb.UsbConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbConfigurationDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbConfigurationDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbConfigurationDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbConfigurationDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbConfigurationDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbConfigurationDescriptor[] = L"Windows.Devices.Usb.UsbConfigurationDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbControlRequestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbControlRequestType ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbControlRequestType_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbControlRequestType_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbControlRequestType[] = L"Windows.Devices.Usb.UsbControlRequestType";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDescriptor[] = L"Windows.Devices.Usb.UsbDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDevice[] = L"Windows.Devices.Usb.UsbDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceClass ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClass_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClass_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceClass[] = L"Windows.Devices.Usb.UsbDeviceClass";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceClasses
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbDeviceClassesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceClasses ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClasses_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceClasses_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceClasses[] = L"Windows.Devices.Usb.UsbDeviceClasses";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbDeviceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbDeviceDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbDeviceDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbDeviceDescriptor[] = L"Windows.Devices.Usb.UsbDeviceDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbEndpointDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbEndpointDescriptor[] = L"Windows.Devices.Usb.UsbEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterface ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterface_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterface_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterface[] = L"Windows.Devices.Usb.UsbInterface";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterfaceDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Usb.IUsbInterfaceDescriptorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterfaceDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterfaceDescriptor[] = L"Windows.Devices.Usb.UsbInterfaceDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterfaceSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterfaceSetting ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceSetting_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterfaceSetting_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterfaceSetting[] = L"Windows.Devices.Usb.UsbInterfaceSetting";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInEndpointDescriptor[] = L"Windows.Devices.Usb.UsbInterruptInEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInEventArgs[] = L"Windows.Devices.Usb.UsbInterruptInEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptInPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptInPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptInPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptInPipe[] = L"Windows.Devices.Usb.UsbInterruptInPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptOutEndpointDescriptor[] = L"Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbInterruptOutPipe
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbInterruptOutPipe ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutPipe_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbInterruptOutPipe_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbInterruptOutPipe[] = L"Windows.Devices.Usb.UsbInterruptOutPipe";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Usb.UsbSetupPacket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Usb.IUsbSetupPacketFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Usb.IUsbSetupPacket ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Usb_UsbSetupPacket_DEFINED
#define RUNTIMECLASS_Windows_Devices_Usb_UsbSetupPacket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Usb_UsbSetupPacket[] = L"Windows.Devices.Usb.UsbSetupPacket";
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
#endif // __windows2Edevices2Eusb_p_h__

#endif // __windows2Edevices2Eusb_h__
