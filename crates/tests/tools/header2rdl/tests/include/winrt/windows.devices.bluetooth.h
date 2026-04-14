
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
#ifndef __windows2Edevices2Ebluetooth_h__
#define __windows2Edevices2Ebluetooth_h__
#ifndef __windows2Edevices2Ebluetooth_p_h__
#define __windows2Edevices2Ebluetooth_p_h__


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
#include "Windows.Devices.Bluetooth.GenericAttributeProfile.h"
#include "Windows.Devices.Bluetooth.Rfcomm.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Devices.Radios.h"
#include "Windows.Networking.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothAdapter;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter ABI::Windows::Devices::Bluetooth::IBluetoothAdapter

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothAdapter2;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2 ABI::Windows::Devices::Bluetooth::IBluetoothAdapter2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothAdapter3;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3 ABI::Windows::Devices::Bluetooth::IBluetoothAdapter3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothAdapter4;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4 ABI::Windows::Devices::Bluetooth::IBluetoothAdapter4

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothAdapterStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics ABI::Windows::Devices::Bluetooth::IBluetoothAdapterStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothClassOfDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDevice

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothClassOfDeviceStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice2;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2 ABI::Windows::Devices::Bluetooth::IBluetoothDevice2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice3;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3 ABI::Windows::Devices::Bluetooth::IBluetoothDevice3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice4;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4 ABI::Windows::Devices::Bluetooth::IBluetoothDevice4

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDevice5;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5 ABI::Windows::Devices::Bluetooth::IBluetoothDevice5

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDeviceId;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId ABI::Windows::Devices::Bluetooth::IBluetoothDeviceId

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDeviceIdStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics ABI::Windows::Devices::Bluetooth::IBluetoothDeviceIdStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDeviceStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics ABI::Windows::Devices::Bluetooth::IBluetoothDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothDeviceStatics2;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2 ABI::Windows::Devices::Bluetooth::IBluetoothDeviceStatics2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEAppearance;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearance

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEAppearanceCategoriesStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearanceCategoriesStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEAppearanceStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearanceStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEAppearanceSubcategoriesStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearanceSubcategoriesStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEConnectionParameters;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionParameters

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEConnectionPhy;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionPhy

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEConnectionPhyInfo;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionPhyInfo

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice2;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2 ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice3;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3 ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice4;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4 ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice4

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice5;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5 ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice5

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDevice6;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6 ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice6

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDeviceStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics ABI::Windows::Devices::Bluetooth::IBluetoothLEDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEDeviceStatics2;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2 ABI::Windows::Devices::Bluetooth::IBluetoothLEDeviceStatics2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEPreferredConnectionParameters;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParameters

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEPreferredConnectionParametersRequest;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParametersRequest

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothLEPreferredConnectionParametersStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParametersStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                interface IBluetoothUuidHelperStatics;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics ABI::Windows::Devices::Bluetooth::IBluetoothUuidHelperStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothAdapter;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("46fce70c-6c07-5a3a-b775-26f99402553f"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*, ABI::Windows::Devices::Bluetooth::IBluetoothAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.BluetoothAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10a10a88-90e0-511a-9a08-d75feb52a19f"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*, ABI::Windows::Devices::Bluetooth::IBluetoothAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.BluetoothAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothAdapter*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b58d8d19-44bd-5ac0-a0d6-1b50800f3181"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.BluetoothDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothDevice*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b2e8cdd1-66aa-5892-85a3-8f0b165e43fc"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.BluetoothDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("375f9d67-74a2-5f91-a11d-169093718d41"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.BluetoothLEDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9156b79f-c54a-5277-8f8b-d2cc43c7e004"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.BluetoothLEDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattDeviceServicesResult;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattDeviceServicesResult;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceServicesResult

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e7c667f6-e874-500f-86ff-760ca6f07a58"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceServicesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("74ab0892-a631-5d6c-b1b4-bd2e1a741a9b"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceServicesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceServicesResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommDeviceServicesResult;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceServicesResult;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServicesResult

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b7f2f74d-bf9c-5721-bf6e-03f1b4409588"))
IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServicesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*> __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("522c25d1-866b-5de4-bd8e-1feb5ae60d47"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceServicesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceServicesResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceAccessStatus : int DeviceAccessStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c00bc2f2-a7f8-5f3f-80d1-2808ef6bca10"))
IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee154d83-805b-53e8-8469-90715036d013"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                class Radio;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                interface IRadio;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CRadios_CIRadio ABI::Windows::Devices::Radios::IRadio

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eac62c40-8dbc-5854-8ba0-b7b9940e7389"))
IAsyncOperation<ABI::Windows::Devices::Radios::Radio*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Radios::Radio*> __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a5c7e3a-80e2-585b-8630-7a8e777f0354"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Radios::Radio*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Radios::Radio*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    class GattDeviceService;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace GenericAttributeProfile {
                    interface IGattDeviceService;
                } /* GenericAttributeProfile */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceService

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8beb3a26-73ca-50f3-a1d3-418c60a9f3b2"))
IIterator<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b192e23-4893-56b2-8eff-439c3ab7fd1f"))
IIterable<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    class RfcommDeviceService;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Rfcomm {
                    interface IRfcommDeviceService;
                } /* Rfcomm */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64ab0132-c64c-5a87-8113-613ef356924c"))
IIterator<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3378e9a6-f6e2-50ea-bfee-b8109631feca"))
IIterable<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

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

#ifndef DEF___FIIterator_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterator_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("afee38e0-f882-5f10-9655-1fc98cc8cce5"))
IIterator<ABI::Windows::Storage::Streams::IBuffer*> : IIterator_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::Streams::IBuffer*> __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterable_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("902972bf-a984-5443-b1c5-2f04a99e1fca"))
IIterable<ABI::Windows::Storage::Streams::IBuffer*> : IIterable_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::Streams::IBuffer*> __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c8e7fdd-a1a1-528a-81d1-296769227a08"))
IVectorView<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*, ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::GattDeviceService*> __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97df6b82-d15c-597e-ba69-492207a1c108"))
IVectorView<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*, ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommDeviceService*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Rfcomm::RfcommDeviceService*> __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fd944562-11d6-5eab-bd72-701993b68fac"))
IVectorView<ABI::Windows::Storage::Streams::IBuffer*> : IVectorView_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::Streams::IBuffer*> __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_USE */

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



#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("604d0c4c-91de-5c2a-935f-362f13eaf800"))
IReference<struct ABI::Windows::Foundation::TimeSpan> : IReference_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::TimeSpan> __FIReference_1_Windows__CFoundation__CTimeSpan_t;
#define __FIReference_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CTimeSpan_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("db56ce1c-5e9f-5138-9227-b1a66d60bc1b"))
ITypedEventHandler<ABI::Windows::Devices::Bluetooth::BluetoothDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Bluetooth.BluetoothDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Bluetooth::BluetoothDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a90661e2-372e-5d1e-bbbb-b8a2ce0e7c4d"))
ITypedEventHandler<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*, ABI::Windows::Devices::Bluetooth::IBluetoothLEDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Bluetooth.BluetoothLEDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Bluetooth::BluetoothLEDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceAccessInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation ABI::Windows::Devices::Enumeration::IDeviceAccessInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation ABI::Windows::Devices::Enumeration::IDeviceInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

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
        namespace Networking {
            class HostName;
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostName;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostName ABI::Windows::Networking::IHostName

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothAddressType : int BluetoothAddressType;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothCacheMode : int BluetoothCacheMode;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothConnectionStatus : int BluetoothConnectionStatus;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothLEPreferredConnectionParametersRequestStatus : int BluetoothLEPreferredConnectionParametersRequestStatus;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothMajorClass : int BluetoothMajorClass;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                typedef enum BluetoothMinorClass : int BluetoothMinorClass;
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
                class BluetoothClassOfDevice;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothDeviceId;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEAppearance;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEConnectionParameters;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEConnectionPhy;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEConnectionPhyInfo;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEPreferredConnectionParameters;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                class BluetoothLEPreferredConnectionParametersRequest;
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothAddressType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothAddressType : int
                {
                    BluetoothAddressType_Public = 0,
                    BluetoothAddressType_Random = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    BluetoothAddressType_Unspecified = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothCacheMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothCacheMode : int
                {
                    BluetoothCacheMode_Cached = 0,
                    BluetoothCacheMode_Uncached = 1,
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothConnectionStatus : int
                {
                    BluetoothConnectionStatus_Disconnected = 0,
                    BluetoothConnectionStatus_Connected = 1,
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothError : int
                {
                    BluetoothError_Success = 0,
                    BluetoothError_RadioNotAvailable = 1,
                    BluetoothError_ResourceInUse = 2,
                    BluetoothError_DeviceNotConnected = 3,
                    BluetoothError_OtherError = 4,
                    BluetoothError_DisabledByPolicy = 5,
                    BluetoothError_NotSupported = 6,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    BluetoothError_DisabledByUser = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    BluetoothError_ConsentRequired = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    BluetoothError_TransportNotSupported = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothLEPreferredConnectionParametersRequestStatus : int
                {
                    BluetoothLEPreferredConnectionParametersRequestStatus_Unspecified = 0,
                    BluetoothLEPreferredConnectionParametersRequestStatus_Success = 1,
                    BluetoothLEPreferredConnectionParametersRequestStatus_DeviceNotAvailable = 2,
                    BluetoothLEPreferredConnectionParametersRequestStatus_AccessDenied = 3,
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothMajorClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothMajorClass : int
                {
                    BluetoothMajorClass_Miscellaneous = 0,
                    BluetoothMajorClass_Computer = 1,
                    BluetoothMajorClass_Phone = 2,
                    BluetoothMajorClass_NetworkAccessPoint = 3,
                    BluetoothMajorClass_AudioVideo = 4,
                    BluetoothMajorClass_Peripheral = 5,
                    BluetoothMajorClass_Imaging = 6,
                    BluetoothMajorClass_Wearable = 7,
                    BluetoothMajorClass_Toy = 8,
                    BluetoothMajorClass_Health = 9,
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothMinorClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothMinorClass : int
                {
                    BluetoothMinorClass_Uncategorized = 0,
                    BluetoothMinorClass_ComputerDesktop = 1,
                    BluetoothMinorClass_ComputerServer = 2,
                    BluetoothMinorClass_ComputerLaptop = 3,
                    BluetoothMinorClass_ComputerHandheld = 4,
                    BluetoothMinorClass_ComputerPalmSize = 5,
                    BluetoothMinorClass_ComputerWearable = 6,
                    BluetoothMinorClass_ComputerTablet = 7,
                    BluetoothMinorClass_PhoneCellular = 1,
                    BluetoothMinorClass_PhoneCordless = 2,
                    BluetoothMinorClass_PhoneSmartPhone = 3,
                    BluetoothMinorClass_PhoneWired = 4,
                    BluetoothMinorClass_PhoneIsdn = 5,
                    BluetoothMinorClass_NetworkFullyAvailable = 0,
                    BluetoothMinorClass_NetworkUsed01To17Percent = 8,
                    BluetoothMinorClass_NetworkUsed17To33Percent = 16,
                    BluetoothMinorClass_NetworkUsed33To50Percent = 24,
                    BluetoothMinorClass_NetworkUsed50To67Percent = 32,
                    BluetoothMinorClass_NetworkUsed67To83Percent = 40,
                    BluetoothMinorClass_NetworkUsed83To99Percent = 48,
                    BluetoothMinorClass_NetworkNoServiceAvailable = 56,
                    BluetoothMinorClass_AudioVideoWearableHeadset = 1,
                    BluetoothMinorClass_AudioVideoHandsFree = 2,
                    BluetoothMinorClass_AudioVideoMicrophone = 4,
                    BluetoothMinorClass_AudioVideoLoudspeaker = 5,
                    BluetoothMinorClass_AudioVideoHeadphones = 6,
                    BluetoothMinorClass_AudioVideoPortableAudio = 7,
                    BluetoothMinorClass_AudioVideoCarAudio = 8,
                    BluetoothMinorClass_AudioVideoSetTopBox = 9,
                    BluetoothMinorClass_AudioVideoHifiAudioDevice = 10,
                    BluetoothMinorClass_AudioVideoVcr = 11,
                    BluetoothMinorClass_AudioVideoVideoCamera = 12,
                    BluetoothMinorClass_AudioVideoCamcorder = 13,
                    BluetoothMinorClass_AudioVideoVideoMonitor = 14,
                    BluetoothMinorClass_AudioVideoVideoDisplayAndLoudspeaker = 15,
                    BluetoothMinorClass_AudioVideoVideoConferencing = 16,
                    BluetoothMinorClass_AudioVideoGamingOrToy = 18,
                    BluetoothMinorClass_PeripheralJoystick = 1,
                    BluetoothMinorClass_PeripheralGamepad = 2,
                    BluetoothMinorClass_PeripheralRemoteControl = 3,
                    BluetoothMinorClass_PeripheralSensing = 4,
                    BluetoothMinorClass_PeripheralDigitizerTablet = 5,
                    BluetoothMinorClass_PeripheralCardReader = 6,
                    BluetoothMinorClass_PeripheralDigitalPen = 7,
                    BluetoothMinorClass_PeripheralHandheldScanner = 8,
                    BluetoothMinorClass_PeripheralHandheldGesture = 9,
                    BluetoothMinorClass_WearableWristwatch = 1,
                    BluetoothMinorClass_WearablePager = 2,
                    BluetoothMinorClass_WearableJacket = 3,
                    BluetoothMinorClass_WearableHelmet = 4,
                    BluetoothMinorClass_WearableGlasses = 5,
                    BluetoothMinorClass_ToyRobot = 1,
                    BluetoothMinorClass_ToyVehicle = 2,
                    BluetoothMinorClass_ToyDoll = 3,
                    BluetoothMinorClass_ToyController = 4,
                    BluetoothMinorClass_ToyGame = 5,
                    BluetoothMinorClass_HealthBloodPressureMonitor = 1,
                    BluetoothMinorClass_HealthThermometer = 2,
                    BluetoothMinorClass_HealthWeighingScale = 3,
                    BluetoothMinorClass_HealthGlucoseMeter = 4,
                    BluetoothMinorClass_HealthPulseOximeter = 5,
                    BluetoothMinorClass_HealthHeartRateMonitor = 6,
                    BluetoothMinorClass_HealthHealthDataDisplay = 7,
                    BluetoothMinorClass_HealthStepCounter = 8,
                    BluetoothMinorClass_HealthBodyCompositionAnalyzer = 9,
                    BluetoothMinorClass_HealthPeakFlowMonitor = 10,
                    BluetoothMinorClass_HealthMedicationMonitor = 11,
                    BluetoothMinorClass_HealthKneeProsthesis = 12,
                    BluetoothMinorClass_HealthAnkleProsthesis = 13,
                    BluetoothMinorClass_HealthGenericHealthManager = 14,
                    BluetoothMinorClass_HealthPersonalMobilityDevice = 15,
                };
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothServiceCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                enum BluetoothServiceCapabilities : unsigned int
                {
                    BluetoothServiceCapabilities_None = 0,
                    BluetoothServiceCapabilities_LimitedDiscoverableMode = 0x1,
                    BluetoothServiceCapabilities_PositioningService = 0x8,
                    BluetoothServiceCapabilities_NetworkingService = 0x10,
                    BluetoothServiceCapabilities_RenderingService = 0x20,
                    BluetoothServiceCapabilities_CapturingService = 0x40,
                    BluetoothServiceCapabilities_ObjectTransferService = 0x80,
                    BluetoothServiceCapabilities_AudioService = 0x100,
                    BluetoothServiceCapabilities_TelephoneService = 0x200,
                    BluetoothServiceCapabilities_InformationService = 0x400,
                };

                DEFINE_ENUM_FLAG_OPERATORS(BluetoothServiceCapabilities)
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("7974f04c-5f7a-4a34-9225-a855f84b1a8b")
                IBluetoothAdapter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddress(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsClassicSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLowEnergySupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPeripheralRoleSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCentralRoleSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAdvertisementOffloadSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRadioAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothAdapter = __uuidof(IBluetoothAdapter);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter2[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("ac94cecc-24d5-41b3-916d-1097c50b102b")
                IBluetoothAdapter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AreClassicSecureConnectionsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AreLowEnergySecureConnectionsSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothAdapter2 = __uuidof(IBluetoothAdapter2);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter3[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("8f8624e0-cba9-5211-9f89-3aac62b4c6b8")
                IBluetoothAdapter3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsExtendedAdvertisingSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAdvertisementDataLength(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothAdapter3 = __uuidof(IBluetoothAdapter3);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter4[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter4";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("f875f3e1-6d9a-5d5e-aee5-a17248e5f6dd")
                IBluetoothAdapter4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsLowEnergyUncoded2MPhySupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLowEnergyCodedPhySupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothAdapter4 = __uuidof(IBluetoothAdapter4);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapterStatics[] = L"Windows.Devices.Bluetooth.IBluetoothAdapterStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("8b02fb6a-ac4c-4741-8661-8eab7d17ea9f")
                IBluetoothAdapterStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothAdapterStatics = __uuidof(IBluetoothAdapterStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothClassOfDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothClassOfDevice[] = L"Windows.Devices.Bluetooth.IBluetoothClassOfDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("d640227e-d7d7-4661-9454-65039ca17a2b")
                IBluetoothClassOfDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RawValue(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MajorClass(
                        ABI::Windows::Devices::Bluetooth::BluetoothMajorClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinorClass(
                        ABI::Windows::Devices::Bluetooth::BluetoothMinorClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceCapabilities(
                        ABI::Windows::Devices::Bluetooth::BluetoothServiceCapabilities* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothClassOfDevice = __uuidof(IBluetoothClassOfDevice);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothClassOfDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("e46135bd-0fa2-416c-91b4-c1e48ca061c1")
                IBluetoothClassOfDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromRawValue(
                        UINT32 rawValue,
                        ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDevice** classOfDevice
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromParts(
                        ABI::Windows::Devices::Bluetooth::BluetoothMajorClass majorClass,
                        ABI::Windows::Devices::Bluetooth::BluetoothMinorClass minorClass,
                        ABI::Windows::Devices::Bluetooth::BluetoothServiceCapabilities serviceCapabilities,
                        ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDevice** classOfDevice
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothClassOfDeviceStatics = __uuidof(IBluetoothClassOfDeviceStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice[] = L"Windows.Devices.Bluetooth.IBluetoothDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("2335b156-90d2-4a04-aef5-0e20b9e6b707")
                IBluetoothDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HostName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClassOfDevice(
                        ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SdpRecords(
                        __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("Use GetRfcommServicesAsync instead of RfcommServices.  For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    virtual HRESULT STDMETHODCALLTYPE get_RfcommServices(
                        __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        ABI::Windows::Devices::Bluetooth::BluetoothConnectionStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddress(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NameChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NameChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SdpRecordsChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SdpRecordsChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionStatusChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDevice = __uuidof(IBluetoothDevice);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice2[] = L"Windows.Devices.Bluetooth.IBluetoothDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("0133f954-b156-4dd0-b1f5-c11bc31a5163")
                IBluetoothDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDevice2 = __uuidof(IBluetoothDevice2);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice3[] = L"Windows.Devices.Bluetooth.IBluetoothDevice3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("57fff78b-651a-4454-b90f-eb21ef0b0d71")
                IBluetoothDevice3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAccessInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRfcommServicesAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRfcommServicesWithCacheModeAsync(
                        ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRfcommServicesForIdAsync(
                        ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRfcommServicesForIdWithCacheModeAsync(
                        ABI::Windows::Devices::Bluetooth::Rfcomm::IRfcommServiceId* serviceId,
                        ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDevice3 = __uuidof(IBluetoothDevice3);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice4[] = L"Windows.Devices.Bluetooth.IBluetoothDevice4";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("817c34ad-0e9c-42b2-a8dc-3e8094940d12")
                IBluetoothDevice4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothDeviceId(
                        ABI::Windows::Devices::Bluetooth::IBluetoothDeviceId** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDevice4 = __uuidof(IBluetoothDevice4);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice5[] = L"Windows.Devices.Bluetooth.IBluetoothDevice5";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("b5e0b385-5e85-4559-a10d-1c7281379f96")
                IBluetoothDevice5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WasSecureConnectionUsedForPairing(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDevice5 = __uuidof(IBluetoothDevice5);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceId[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceId";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("c17949af-57c1-4642-bcce-e6c06b20ae76")
                IBluetoothDeviceId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsClassicDevice(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLowEnergyDevice(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDeviceId = __uuidof(IBluetoothDeviceId);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceIdStatics[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("a7884e67-3efb-4f31-bbc2-810e09977404")
                IBluetoothDeviceIdStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING deviceId,
                        ABI::Windows::Devices::Bluetooth::IBluetoothDeviceId** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDeviceIdStatics = __uuidof(IBluetoothDeviceIdStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("0991df51-57db-4725-bbd7-84f64327ec2c")
                IBluetoothDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromHostNameAsync(
                        ABI::Windows::Networking::IHostName* hostName,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromBluetoothAddressAsync(
                        UINT64 address,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* deviceSelector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDeviceStatics = __uuidof(IBluetoothDeviceStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceStatics2[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("c29e8e2f-4e14-4477-aa1b-b8b47e5b7ece")
                IBluetoothDeviceStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromPairingState(
                        boolean pairingState,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromConnectionStatus(
                        ABI::Windows::Devices::Bluetooth::BluetoothConnectionStatus connectionStatus,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromDeviceName(
                        HSTRING deviceName,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromBluetoothAddress(
                        UINT64 bluetoothAddress,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromClassOfDevice(
                        ABI::Windows::Devices::Bluetooth::IBluetoothClassOfDevice* classOfDevice,
                        HSTRING* deviceSelector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothDeviceStatics2 = __uuidof(IBluetoothDeviceStatics2);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearance[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearance";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("5d2079f2-66a8-4258-985e-02b4d9509f18")
                IBluetoothLEAppearance : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RawValue(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Category(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubCategory(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEAppearance = __uuidof(IBluetoothLEAppearance);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceCategoriesStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("6d4d54fe-046a-4185-aab6-824cf0610861")
                IBluetoothLEAppearanceCategoriesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uncategorized(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Phone(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Computer(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Watch(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clock(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Display(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteControl(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EyeGlasses(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keyring(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaPlayer(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BarcodeScanner(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thermometer(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeartRate(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BloodPressure(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HumanInterfaceDevice(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GlucoseMeter(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RunningWalking(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cycling(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PulseOximeter(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WeightScale(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutdoorSportActivity(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEAppearanceCategoriesStatics = __uuidof(IBluetoothLEAppearanceCategoriesStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("a193c0c7-4504-4f4a-9ba5-cd1054e5e065")
                IBluetoothLEAppearanceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromRawValue(
                        UINT16 rawValue,
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearance** appearance
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromParts(
                        UINT16 appearanceCategory,
                        UINT16 appearanceSubCategory,
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearance** appearance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEAppearanceStatics = __uuidof(IBluetoothLEAppearanceStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceSubcategoriesStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("e57ba606-2144-415a-8312-71ccf291f8d1")
                IBluetoothLEAppearanceSubcategoriesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Generic(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SportsWatch(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThermometerEar(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeartRateBelt(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BloodPressureArm(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BloodPressureWrist(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keyboard(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mouse(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Joystick(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gamepad(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DigitizerTablet(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CardReader(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DigitalPen(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BarcodeScanner(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RunningWalkingInShoe(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RunningWalkingOnShoe(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RunningWalkingOnHip(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CyclingComputer(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CyclingSpeedSensor(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CyclingCadenceSensor(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CyclingPowerSensor(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CyclingSpeedCadenceSensor(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OximeterFingertip(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OximeterWristWorn(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocationDisplay(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocationNavigationDisplay(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocationPod(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocationNavigationPod(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEAppearanceSubcategoriesStatics = __uuidof(IBluetoothLEAppearanceSubcategoriesStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionParameters[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("33cb0771-8da9-508f-a366-1ca388c929ab")
                IBluetoothLEConnectionParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LinkTimeout(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionLatency(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionInterval(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEConnectionParameters = __uuidof(IBluetoothLEConnectionParameters);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionPhy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionPhy[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("781e5e48-621e-5a7e-8be6-1b9561ff63c9")
                IBluetoothLEConnectionPhy : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransmitInfo(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionPhyInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReceiveInfo(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionPhyInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEConnectionPhy = __uuidof(IBluetoothLEConnectionPhy);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionPhyInfo[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("9a100bdd-602e-5c27-a1ae-b230015a6394")
                IBluetoothLEConnectionPhyInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsUncoded1MPhy(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsUncoded2MPhy(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCodedPhy(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEConnectionPhyInfo = __uuidof(IBluetoothLEConnectionPhyInfo);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("b5ee2f7b-4ad8-4642-ac48-80a0b500e887")
                IBluetoothLEDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("Use GetGattServicesAsync instead of GattServices.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    virtual HRESULT STDMETHODCALLTYPE get_GattServices(
                        __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        ABI::Windows::Devices::Bluetooth::BluetoothConnectionStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddress(
                        UINT64* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("Use GetGattServicesForUuidAsync instead of GetGattService.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    virtual HRESULT STDMETHODCALLTYPE GetGattService(
                        GUID serviceUuid,
                        ABI::Windows::Devices::Bluetooth::GenericAttributeProfile::IGattDeviceService** service
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NameChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NameChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_GattServicesChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_GattServicesChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionStatusChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice = __uuidof(IBluetoothLEDevice);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice2[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("26f062b3-7aee-4d31-baba-b1b9775f5916")
                IBluetoothLEDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Appearance(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddressType(
                        ABI::Windows::Devices::Bluetooth::BluetoothAddressType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice2 = __uuidof(IBluetoothLEDevice2);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice3[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("aee9e493-44ac-40dc-af33-b2c13c01ca46")
                IBluetoothLEDevice3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAccessInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGattServicesAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGattServicesWithCacheModeAsync(
                        ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGattServicesForUuidAsync(
                        GUID serviceUuid,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGattServicesForUuidWithCacheModeAsync(
                        GUID serviceUuid,
                        ABI::Windows::Devices::Bluetooth::BluetoothCacheMode cacheMode,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice3 = __uuidof(IBluetoothLEDevice3);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice4[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice4";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("2b605031-2248-4b2f-acf0-7cee36fc5870")
                IBluetoothLEDevice4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BluetoothDeviceId(
                        ABI::Windows::Devices::Bluetooth::IBluetoothDeviceId** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice4 = __uuidof(IBluetoothLEDevice4);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice5[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice5";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("9d6a1260-5287-458e-95ba-17c8b7bb326e")
                IBluetoothLEDevice5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WasSecureConnectionUsedForPairing(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice5 = __uuidof(IBluetoothLEDevice5);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice6[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice6";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("ca7190ef-0cae-573c-a1ca-e1fc5bfc39e2")
                IBluetoothLEDevice6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetConnectionParameters(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionParameters** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConnectionPhy(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEConnectionPhy** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPreferredConnectionParameters(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParameters* preferredConnectionParameters,
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParametersRequest** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionParametersChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionParametersChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionPhyChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionPhyChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDevice6 = __uuidof(IBluetoothLEDevice6);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("c8cf1a19-f0b6-4bf0-8689-41303de2d9f4")
                IBluetoothLEDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromBluetoothAddressAsync(
                        UINT64 bluetoothAddress,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDeviceStatics = __uuidof(IBluetoothLEDeviceStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDeviceStatics2[] = L"Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("5f12c06b-3bac-43e8-ad16-563271bd41c2")
                IBluetoothLEDeviceStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromPairingState(
                        boolean pairingState,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromConnectionStatus(
                        ABI::Windows::Devices::Bluetooth::BluetoothConnectionStatus connectionStatus,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromDeviceName(
                        HSTRING deviceName,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromBluetoothAddress(
                        UINT64 bluetoothAddress,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(
                        UINT64 bluetoothAddress,
                        ABI::Windows::Devices::Bluetooth::BluetoothAddressType bluetoothAddressType,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromAppearance(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEAppearance* appearance,
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromBluetoothAddressWithBluetoothAddressTypeAsync(
                        UINT64 bluetoothAddress,
                        ABI::Windows::Devices::Bluetooth::BluetoothAddressType bluetoothAddressType,
                        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEDeviceStatics2 = __uuidof(IBluetoothLEDeviceStatics2);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParameters[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("f2f44344-7372-5f7b-9b34-29c944f5a715")
                IBluetoothLEPreferredConnectionParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LinkTimeout(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionLatency(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinConnectionInterval(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxConnectionInterval(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEPreferredConnectionParameters = __uuidof(IBluetoothLEPreferredConnectionParameters);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParametersRequest[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("8a375276-a528-5266-b661-cce6a5ff9739")
                IBluetoothLEPreferredConnectionParametersRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Bluetooth::BluetoothLEPreferredConnectionParametersRequestStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEPreferredConnectionParametersRequest = __uuidof(IBluetoothLEPreferredConnectionParametersRequest);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParametersStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("0e3e8edc-2751-55aa-a838-8faeee818d72")
                IBluetoothLEPreferredConnectionParametersStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Balanced(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParameters** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThroughputOptimized(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParameters** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerOptimized(
                        ABI::Windows::Devices::Bluetooth::IBluetoothLEPreferredConnectionParameters** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothLEPreferredConnectionParametersStatics = __uuidof(IBluetoothLEPreferredConnectionParametersStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothSignalStrengthFilter[] = L"Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("df7b7391-6bb5-4cfe-90b1-5d7324edcf7f")
                IBluetoothSignalStrengthFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InRangeThresholdInDBm(
                        __FIReference_1_short** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InRangeThresholdInDBm(
                        __FIReference_1_short* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutOfRangeThresholdInDBm(
                        __FIReference_1_short** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutOfRangeThresholdInDBm(
                        __FIReference_1_short* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutOfRangeTimeout(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutOfRangeTimeout(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SamplingInterval(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SamplingInterval(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothSignalStrengthFilter = __uuidof(IBluetoothSignalStrengthFilter);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothUuidHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothUuidHelperStatics[] = L"Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                MIDL_INTERFACE("17df0cd8-cf74-4b21-afe6-f57a11bcdea0")
                IBluetoothUuidHelperStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromShortId(
                        UINT32 shortId,
                        GUID* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetShortId(
                        GUID uuid,
                        __FIReference_1_UINT32** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBluetoothUuidHelperStatics = __uuidof(IBluetoothUuidHelperStatics);
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothAdapterStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothAdapter ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothAdapter2
 *    Windows.Devices.Bluetooth.IBluetoothAdapter3
 *    Windows.Devices.Bluetooth.IBluetoothAdapter4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothAdapter[] = L"Windows.Devices.Bluetooth.BluetoothAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothClassOfDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothClassOfDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothClassOfDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothClassOfDevice[] = L"Windows.Devices.Bluetooth.BluetoothClassOfDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothDevice ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothDevice2
 *    Windows.Devices.Bluetooth.IBluetoothDevice3
 *    Windows.Devices.Bluetooth.IBluetoothDevice4
 *    Windows.Devices.Bluetooth.IBluetoothDevice5
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothDevice[] = L"Windows.Devices.Bluetooth.BluetoothDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothDeviceId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDeviceId_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDeviceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothDeviceId[] = L"Windows.Devices.Bluetooth.BluetoothDeviceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEAppearance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearance_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearance[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionPhy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice2
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice3
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice4
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice5
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice6
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEDevice[] = L"Windows.Devices.Bluetooth.BluetoothLEDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters[] = L"Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest[] = L"Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter[] = L"Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothUuidHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothUuidHelper_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothUuidHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothUuidHelper[] = L"Windows.Devices.Bluetooth.BluetoothUuidHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2 __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceServicesResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceServicesResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CRadios_CIRadio __x_ABI_CWindows_CDevices_CRadios_CIRadio;

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStreams__CIBuffer __FIIterator_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterator_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterator_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIBuffer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStreams__CIBuffer __FIIterable_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterable_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIIterator_1_Windows__CStorage__CStreams__CIBuffer** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterable_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStreams__CIBuffer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommDeviceService** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIVectorView_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
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

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CTimeSpan __FIReference_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIReference_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothLEPreferredConnectionParametersRequestStatus __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothLEPreferredConnectionParametersRequestStatus;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMajorClass __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMajorClass;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMinorClass __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMinorClass;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities;

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothAddressType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType
{
    BluetoothAddressType_Public = 0,
    BluetoothAddressType_Random = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    BluetoothAddressType_Unspecified = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothCacheMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode
{
    BluetoothCacheMode_Cached = 0,
    BluetoothCacheMode_Uncached = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus
{
    BluetoothConnectionStatus_Disconnected = 0,
    BluetoothConnectionStatus_Connected = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError
{
    BluetoothError_Success = 0,
    BluetoothError_RadioNotAvailable = 1,
    BluetoothError_ResourceInUse = 2,
    BluetoothError_DeviceNotConnected = 3,
    BluetoothError_OtherError = 4,
    BluetoothError_DisabledByPolicy = 5,
    BluetoothError_NotSupported = 6,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    BluetoothError_DisabledByUser = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    BluetoothError_ConsentRequired = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    BluetoothError_TransportNotSupported = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothLEPreferredConnectionParametersRequestStatus
{
    BluetoothLEPreferredConnectionParametersRequestStatus_Unspecified = 0,
    BluetoothLEPreferredConnectionParametersRequestStatus_Success = 1,
    BluetoothLEPreferredConnectionParametersRequestStatus_DeviceNotAvailable = 2,
    BluetoothLEPreferredConnectionParametersRequestStatus_AccessDenied = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothMajorClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMajorClass
{
    BluetoothMajorClass_Miscellaneous = 0,
    BluetoothMajorClass_Computer = 1,
    BluetoothMajorClass_Phone = 2,
    BluetoothMajorClass_NetworkAccessPoint = 3,
    BluetoothMajorClass_AudioVideo = 4,
    BluetoothMajorClass_Peripheral = 5,
    BluetoothMajorClass_Imaging = 6,
    BluetoothMajorClass_Wearable = 7,
    BluetoothMajorClass_Toy = 8,
    BluetoothMajorClass_Health = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothMinorClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMinorClass
{
    BluetoothMinorClass_Uncategorized = 0,
    BluetoothMinorClass_ComputerDesktop = 1,
    BluetoothMinorClass_ComputerServer = 2,
    BluetoothMinorClass_ComputerLaptop = 3,
    BluetoothMinorClass_ComputerHandheld = 4,
    BluetoothMinorClass_ComputerPalmSize = 5,
    BluetoothMinorClass_ComputerWearable = 6,
    BluetoothMinorClass_ComputerTablet = 7,
    BluetoothMinorClass_PhoneCellular = 1,
    BluetoothMinorClass_PhoneCordless = 2,
    BluetoothMinorClass_PhoneSmartPhone = 3,
    BluetoothMinorClass_PhoneWired = 4,
    BluetoothMinorClass_PhoneIsdn = 5,
    BluetoothMinorClass_NetworkFullyAvailable = 0,
    BluetoothMinorClass_NetworkUsed01To17Percent = 8,
    BluetoothMinorClass_NetworkUsed17To33Percent = 16,
    BluetoothMinorClass_NetworkUsed33To50Percent = 24,
    BluetoothMinorClass_NetworkUsed50To67Percent = 32,
    BluetoothMinorClass_NetworkUsed67To83Percent = 40,
    BluetoothMinorClass_NetworkUsed83To99Percent = 48,
    BluetoothMinorClass_NetworkNoServiceAvailable = 56,
    BluetoothMinorClass_AudioVideoWearableHeadset = 1,
    BluetoothMinorClass_AudioVideoHandsFree = 2,
    BluetoothMinorClass_AudioVideoMicrophone = 4,
    BluetoothMinorClass_AudioVideoLoudspeaker = 5,
    BluetoothMinorClass_AudioVideoHeadphones = 6,
    BluetoothMinorClass_AudioVideoPortableAudio = 7,
    BluetoothMinorClass_AudioVideoCarAudio = 8,
    BluetoothMinorClass_AudioVideoSetTopBox = 9,
    BluetoothMinorClass_AudioVideoHifiAudioDevice = 10,
    BluetoothMinorClass_AudioVideoVcr = 11,
    BluetoothMinorClass_AudioVideoVideoCamera = 12,
    BluetoothMinorClass_AudioVideoCamcorder = 13,
    BluetoothMinorClass_AudioVideoVideoMonitor = 14,
    BluetoothMinorClass_AudioVideoVideoDisplayAndLoudspeaker = 15,
    BluetoothMinorClass_AudioVideoVideoConferencing = 16,
    BluetoothMinorClass_AudioVideoGamingOrToy = 18,
    BluetoothMinorClass_PeripheralJoystick = 1,
    BluetoothMinorClass_PeripheralGamepad = 2,
    BluetoothMinorClass_PeripheralRemoteControl = 3,
    BluetoothMinorClass_PeripheralSensing = 4,
    BluetoothMinorClass_PeripheralDigitizerTablet = 5,
    BluetoothMinorClass_PeripheralCardReader = 6,
    BluetoothMinorClass_PeripheralDigitalPen = 7,
    BluetoothMinorClass_PeripheralHandheldScanner = 8,
    BluetoothMinorClass_PeripheralHandheldGesture = 9,
    BluetoothMinorClass_WearableWristwatch = 1,
    BluetoothMinorClass_WearablePager = 2,
    BluetoothMinorClass_WearableJacket = 3,
    BluetoothMinorClass_WearableHelmet = 4,
    BluetoothMinorClass_WearableGlasses = 5,
    BluetoothMinorClass_ToyRobot = 1,
    BluetoothMinorClass_ToyVehicle = 2,
    BluetoothMinorClass_ToyDoll = 3,
    BluetoothMinorClass_ToyController = 4,
    BluetoothMinorClass_ToyGame = 5,
    BluetoothMinorClass_HealthBloodPressureMonitor = 1,
    BluetoothMinorClass_HealthThermometer = 2,
    BluetoothMinorClass_HealthWeighingScale = 3,
    BluetoothMinorClass_HealthGlucoseMeter = 4,
    BluetoothMinorClass_HealthPulseOximeter = 5,
    BluetoothMinorClass_HealthHeartRateMonitor = 6,
    BluetoothMinorClass_HealthHealthDataDisplay = 7,
    BluetoothMinorClass_HealthStepCounter = 8,
    BluetoothMinorClass_HealthBodyCompositionAnalyzer = 9,
    BluetoothMinorClass_HealthPeakFlowMonitor = 10,
    BluetoothMinorClass_HealthMedicationMonitor = 11,
    BluetoothMinorClass_HealthKneeProsthesis = 12,
    BluetoothMinorClass_HealthAnkleProsthesis = 13,
    BluetoothMinorClass_HealthGenericHealthManager = 14,
    BluetoothMinorClass_HealthPersonalMobilityDevice = 15,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.BluetoothServiceCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities
{
    BluetoothServiceCapabilities_None = 0,
    BluetoothServiceCapabilities_LimitedDiscoverableMode = 0x1,
    BluetoothServiceCapabilities_PositioningService = 0x8,
    BluetoothServiceCapabilities_NetworkingService = 0x10,
    BluetoothServiceCapabilities_RenderingService = 0x20,
    BluetoothServiceCapabilities_CapturingService = 0x40,
    BluetoothServiceCapabilities_ObjectTransferService = 0x80,
    BluetoothServiceCapabilities_AudioService = 0x100,
    BluetoothServiceCapabilities_TelephoneService = 0x200,
    BluetoothServiceCapabilities_InformationService = 0x400,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IsClassicSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLowEnergySupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPeripheralRoleSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCentralRoleSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAdvertisementOffloadSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetRadioAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter* This,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_BluetoothAddress(This, value) \
    ((This)->lpVtbl->get_BluetoothAddress(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_IsClassicSupported(This, value) \
    ((This)->lpVtbl->get_IsClassicSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_IsLowEnergySupported(This, value) \
    ((This)->lpVtbl->get_IsLowEnergySupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_IsPeripheralRoleSupported(This, value) \
    ((This)->lpVtbl->get_IsPeripheralRoleSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_IsCentralRoleSupported(This, value) \
    ((This)->lpVtbl->get_IsCentralRoleSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_get_IsAdvertisementOffloadSupported(This, value) \
    ((This)->lpVtbl->get_IsAdvertisementOffloadSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_GetRadioAsync(This, operation) \
    ((This)->lpVtbl->GetRadioAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter2[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AreClassicSecureConnectionsSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AreLowEnergySecureConnectionsSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_get_AreClassicSecureConnectionsSupported(This, value) \
    ((This)->lpVtbl->get_AreClassicSecureConnectionsSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_get_AreLowEnergySecureConnectionsSupported(This, value) \
    ((This)->lpVtbl->get_AreLowEnergySecureConnectionsSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter3[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsExtendedAdvertisingSupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxAdvertisementDataLength)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_get_IsExtendedAdvertisingSupported(This, value) \
    ((This)->lpVtbl->get_IsExtendedAdvertisingSupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_get_MaxAdvertisementDataLength(This, value) \
    ((This)->lpVtbl->get_MaxAdvertisementDataLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapter4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapter4[] = L"Windows.Devices.Bluetooth.IBluetoothAdapter4";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsLowEnergyUncoded2MPhySupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLowEnergyCodedPhySupported)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_get_IsLowEnergyUncoded2MPhySupported(This, value) \
    ((This)->lpVtbl->get_IsLowEnergyUncoded2MPhySupported(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_get_IsLowEnergyCodedPhySupported(This, value) \
    ((This)->lpVtbl->get_IsLowEnergyCodedPhySupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapter4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothAdapterStatics[] = L"Windows.Devices.Bluetooth.IBluetoothAdapterStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter** operation);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothAdapter** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothClassOfDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothClassOfDevice[] = L"Windows.Devices.Bluetooth.IBluetoothClassOfDevice";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RawValue)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MajorClass)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMajorClass* value);
    HRESULT (STDMETHODCALLTYPE* get_MinorClass)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMinorClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceCapabilities)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_get_RawValue(This, value) \
    ((This)->lpVtbl->get_RawValue(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_get_MajorClass(This, value) \
    ((This)->lpVtbl->get_MajorClass(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_get_MinorClass(This, value) \
    ((This)->lpVtbl->get_MinorClass(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_get_ServiceCapabilities(This, value) \
    ((This)->lpVtbl->get_ServiceCapabilities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothClassOfDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromRawValue)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        UINT32 rawValue,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice** classOfDevice);
    HRESULT (STDMETHODCALLTYPE* FromParts)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMajorClass majorClass,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothMinorClass minorClass,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothServiceCapabilities serviceCapabilities,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice** classOfDevice);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FromRawValue(This, rawValue, classOfDevice) \
    ((This)->lpVtbl->FromRawValue(This, rawValue, classOfDevice))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_FromParts(This, majorClass, minorClass, serviceCapabilities, classOfDevice) \
    ((This)->lpVtbl->FromParts(This, majorClass, minorClass, serviceCapabilities, classOfDevice))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice[] = L"Windows.Devices.Bluetooth.IBluetoothDevice";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HostName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ClassOfDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_SdpRecords)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __FIVectorView_1_Windows__CStorage__CStreams__CIBuffer** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Use GetRfcommServicesAsync instead of RfcommServices.  For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_RfcommServices)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceService** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionStatus)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* add_NameChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NameChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SdpRecordsChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SdpRecordsChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionStatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionStatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_HostName(This, value) \
    ((This)->lpVtbl->get_HostName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_ClassOfDevice(This, value) \
    ((This)->lpVtbl->get_ClassOfDevice(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_SdpRecords(This, value) \
    ((This)->lpVtbl->get_SdpRecords(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Use GetRfcommServicesAsync instead of RfcommServices.  For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_RfcommServices(This, value) \
    ((This)->lpVtbl->get_RfcommServices(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_ConnectionStatus(This, value) \
    ((This)->lpVtbl->get_ConnectionStatus(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_get_BluetoothAddress(This, value) \
    ((This)->lpVtbl->get_BluetoothAddress(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_add_NameChanged(This, handler, token) \
    ((This)->lpVtbl->add_NameChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_remove_NameChanged(This, token) \
    ((This)->lpVtbl->remove_NameChanged(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_add_SdpRecordsChanged(This, handler, token) \
    ((This)->lpVtbl->add_SdpRecordsChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_remove_SdpRecordsChanged(This, token) \
    ((This)->lpVtbl->remove_SdpRecordsChanged(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_add_ConnectionStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_remove_ConnectionStatusChanged(This, token) \
    ((This)->lpVtbl->remove_ConnectionStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice2[] = L"Windows.Devices.Bluetooth.IBluetoothDevice2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice3[] = L"Windows.Devices.Bluetooth.IBluetoothDevice3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAccessInformation)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** value);
    HRESULT (STDMETHODCALLTYPE* GetRfcommServicesAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetRfcommServicesWithCacheModeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetRfcommServicesForIdAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetRfcommServicesForIdWithCacheModeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CRfcomm_CIRfcommServiceId* serviceId,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CRfcomm__CRfcommDeviceServicesResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_get_DeviceAccessInformation(This, value) \
    ((This)->lpVtbl->get_DeviceAccessInformation(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_RequestAccessAsync(This, value) \
    ((This)->lpVtbl->RequestAccessAsync(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetRfcommServicesAsync(This, operation) \
    ((This)->lpVtbl->GetRfcommServicesAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetRfcommServicesWithCacheModeAsync(This, cacheMode, operation) \
    ((This)->lpVtbl->GetRfcommServicesWithCacheModeAsync(This, cacheMode, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetRfcommServicesForIdAsync(This, serviceId, operation) \
    ((This)->lpVtbl->GetRfcommServicesForIdAsync(This, serviceId, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_GetRfcommServicesForIdWithCacheModeAsync(This, serviceId, cacheMode, operation) \
    ((This)->lpVtbl->GetRfcommServicesForIdWithCacheModeAsync(This, serviceId, cacheMode, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice4[] = L"Windows.Devices.Bluetooth.IBluetoothDevice4";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothDeviceId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_get_BluetoothDeviceId(This, value) \
    ((This)->lpVtbl->get_BluetoothDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDevice5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDevice5[] = L"Windows.Devices.Bluetooth.IBluetoothDevice5";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WasSecureConnectionUsedForPairing)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_get_WasSecureConnectionUsedForPairing(This, value) \
    ((This)->lpVtbl->get_WasSecureConnectionUsedForPairing(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDevice5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceId[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceId";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsClassicDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLowEnergyDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_get_IsClassicDevice(This, value) \
    ((This)->lpVtbl->get_IsClassicDevice(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_get_IsLowEnergyDevice(This, value) \
    ((This)->lpVtbl->get_IsLowEnergyDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceIdStatics[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_FromId(This, deviceId, result) \
    ((This)->lpVtbl->FromId(This, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation);
    HRESULT (STDMETHODCALLTYPE* FromHostNameAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        __x_ABI_CWindows_CNetworking_CIHostName* hostName,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation);
    HRESULT (STDMETHODCALLTYPE* FromBluetoothAddressAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        UINT64 address,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothDevice** operation);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics* This,
        HSTRING* deviceSelector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FromHostNameAsync(This, hostName, operation) \
    ((This)->lpVtbl->FromHostNameAsync(This, hostName, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_FromBluetoothAddressAsync(This, address, operation) \
    ((This)->lpVtbl->FromBluetoothAddressAsync(This, address, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_GetDeviceSelector(This, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelector(This, deviceSelector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothDeviceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothDeviceStatics2[] = L"Windows.Devices.Bluetooth.IBluetoothDeviceStatics2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromPairingState)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        boolean pairingState,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromConnectionStatus)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus connectionStatus,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromDeviceName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        HSTRING deviceName,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromBluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        UINT64 bluetoothAddress,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromClassOfDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothClassOfDevice* classOfDevice,
        HSTRING* deviceSelector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetDeviceSelectorFromPairingState(This, pairingState, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromPairingState(This, pairingState, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetDeviceSelectorFromConnectionStatus(This, connectionStatus, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromConnectionStatus(This, connectionStatus, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetDeviceSelectorFromDeviceName(This, deviceName, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromDeviceName(This, deviceName, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetDeviceSelectorFromBluetoothAddress(This, bluetoothAddress, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromBluetoothAddress(This, bluetoothAddress, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_GetDeviceSelectorFromClassOfDevice(This, classOfDevice, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromClassOfDevice(This, classOfDevice, deviceSelector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearance[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearance";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RawValue)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_SubCategory)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_get_RawValue(This, value) \
    ((This)->lpVtbl->get_RawValue(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_get_SubCategory(This, value) \
    ((This)->lpVtbl->get_SubCategory(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceCategoriesStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uncategorized)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Phone)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Computer)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Watch)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Clock)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Display)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteControl)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_EyeGlasses)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Keyring)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaPlayer)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BarcodeScanner)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Thermometer)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HeartRate)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BloodPressure)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HumanInterfaceDevice)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_GlucoseMeter)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RunningWalking)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Cycling)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_PulseOximeter)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_WeightScale)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_OutdoorSportActivity)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Uncategorized(This, value) \
    ((This)->lpVtbl->get_Uncategorized(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Phone(This, value) \
    ((This)->lpVtbl->get_Phone(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Computer(This, value) \
    ((This)->lpVtbl->get_Computer(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Watch(This, value) \
    ((This)->lpVtbl->get_Watch(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Clock(This, value) \
    ((This)->lpVtbl->get_Clock(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Display(This, value) \
    ((This)->lpVtbl->get_Display(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_RemoteControl(This, value) \
    ((This)->lpVtbl->get_RemoteControl(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_EyeGlasses(This, value) \
    ((This)->lpVtbl->get_EyeGlasses(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Keyring(This, value) \
    ((This)->lpVtbl->get_Keyring(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_MediaPlayer(This, value) \
    ((This)->lpVtbl->get_MediaPlayer(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_BarcodeScanner(This, value) \
    ((This)->lpVtbl->get_BarcodeScanner(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Thermometer(This, value) \
    ((This)->lpVtbl->get_Thermometer(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_HeartRate(This, value) \
    ((This)->lpVtbl->get_HeartRate(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_BloodPressure(This, value) \
    ((This)->lpVtbl->get_BloodPressure(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_HumanInterfaceDevice(This, value) \
    ((This)->lpVtbl->get_HumanInterfaceDevice(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_GlucoseMeter(This, value) \
    ((This)->lpVtbl->get_GlucoseMeter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_RunningWalking(This, value) \
    ((This)->lpVtbl->get_RunningWalking(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_Cycling(This, value) \
    ((This)->lpVtbl->get_Cycling(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_PulseOximeter(This, value) \
    ((This)->lpVtbl->get_PulseOximeter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_WeightScale(This, value) \
    ((This)->lpVtbl->get_WeightScale(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_get_OutdoorSportActivity(This, value) \
    ((This)->lpVtbl->get_OutdoorSportActivity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceCategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromRawValue)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        UINT16 rawValue,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance** appearance);
    HRESULT (STDMETHODCALLTYPE* FromParts)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics* This,
        UINT16 appearanceCategory,
        UINT16 appearanceSubCategory,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance** appearance);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FromRawValue(This, rawValue, appearance) \
    ((This)->lpVtbl->FromRawValue(This, rawValue, appearance))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_FromParts(This, appearanceCategory, appearanceSubCategory, appearance) \
    ((This)->lpVtbl->FromParts(This, appearanceCategory, appearanceSubCategory, appearance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEAppearanceSubcategoriesStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Generic)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_SportsWatch)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ThermometerEar)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HeartRateBelt)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BloodPressureArm)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BloodPressureWrist)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Keyboard)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Mouse)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Joystick)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Gamepad)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_DigitizerTablet)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CardReader)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_DigitalPen)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BarcodeScanner)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RunningWalkingInShoe)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RunningWalkingOnShoe)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RunningWalkingOnHip)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CyclingComputer)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CyclingSpeedSensor)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CyclingCadenceSensor)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CyclingPowerSensor)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_CyclingSpeedCadenceSensor)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_OximeterFingertip)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_OximeterWristWorn)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LocationDisplay)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LocationNavigationDisplay)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LocationPod)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LocationNavigationPod)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_Generic(This, value) \
    ((This)->lpVtbl->get_Generic(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_SportsWatch(This, value) \
    ((This)->lpVtbl->get_SportsWatch(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_ThermometerEar(This, value) \
    ((This)->lpVtbl->get_ThermometerEar(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_HeartRateBelt(This, value) \
    ((This)->lpVtbl->get_HeartRateBelt(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_BloodPressureArm(This, value) \
    ((This)->lpVtbl->get_BloodPressureArm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_BloodPressureWrist(This, value) \
    ((This)->lpVtbl->get_BloodPressureWrist(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_Keyboard(This, value) \
    ((This)->lpVtbl->get_Keyboard(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_Mouse(This, value) \
    ((This)->lpVtbl->get_Mouse(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_Joystick(This, value) \
    ((This)->lpVtbl->get_Joystick(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_Gamepad(This, value) \
    ((This)->lpVtbl->get_Gamepad(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_DigitizerTablet(This, value) \
    ((This)->lpVtbl->get_DigitizerTablet(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CardReader(This, value) \
    ((This)->lpVtbl->get_CardReader(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_DigitalPen(This, value) \
    ((This)->lpVtbl->get_DigitalPen(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_BarcodeScanner(This, value) \
    ((This)->lpVtbl->get_BarcodeScanner(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_RunningWalkingInShoe(This, value) \
    ((This)->lpVtbl->get_RunningWalkingInShoe(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_RunningWalkingOnShoe(This, value) \
    ((This)->lpVtbl->get_RunningWalkingOnShoe(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_RunningWalkingOnHip(This, value) \
    ((This)->lpVtbl->get_RunningWalkingOnHip(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CyclingComputer(This, value) \
    ((This)->lpVtbl->get_CyclingComputer(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CyclingSpeedSensor(This, value) \
    ((This)->lpVtbl->get_CyclingSpeedSensor(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CyclingCadenceSensor(This, value) \
    ((This)->lpVtbl->get_CyclingCadenceSensor(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CyclingPowerSensor(This, value) \
    ((This)->lpVtbl->get_CyclingPowerSensor(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_CyclingSpeedCadenceSensor(This, value) \
    ((This)->lpVtbl->get_CyclingSpeedCadenceSensor(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_OximeterFingertip(This, value) \
    ((This)->lpVtbl->get_OximeterFingertip(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_OximeterWristWorn(This, value) \
    ((This)->lpVtbl->get_OximeterWristWorn(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_LocationDisplay(This, value) \
    ((This)->lpVtbl->get_LocationDisplay(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_LocationNavigationDisplay(This, value) \
    ((This)->lpVtbl->get_LocationNavigationDisplay(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_LocationPod(This, value) \
    ((This)->lpVtbl->get_LocationPod(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_get_LocationNavigationPod(This, value) \
    ((This)->lpVtbl->get_LocationNavigationPod(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearanceSubcategoriesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionParameters[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LinkTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionLatency)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParametersVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_get_LinkTimeout(This, value) \
    ((This)->lpVtbl->get_LinkTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_get_ConnectionLatency(This, value) \
    ((This)->lpVtbl->get_ConnectionLatency(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_get_ConnectionInterval(This, value) \
    ((This)->lpVtbl->get_ConnectionInterval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionPhy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionPhy[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransmitInfo)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ReceiveInfo)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_get_TransmitInfo(This, value) \
    ((This)->lpVtbl->get_TransmitInfo(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_get_ReceiveInfo(This, value) \
    ((This)->lpVtbl->get_ReceiveInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEConnectionPhyInfo[] = L"Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsUncoded1MPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsUncoded2MPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCodedPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfoVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_get_IsUncoded1MPhy(This, value) \
    ((This)->lpVtbl->get_IsUncoded1MPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_get_IsUncoded2MPhy(This, value) \
    ((This)->lpVtbl->get_IsUncoded2MPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_get_IsCodedPhy(This, value) \
    ((This)->lpVtbl->get_IsCodedPhy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhyInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use GetGattServicesAsync instead of GattServices.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_GattServices)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceService** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionStatus)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        UINT64* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use GetGattServicesForUuidAsync instead of GetGattService.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* GetGattService)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        GUID serviceUuid,
        __x_ABI_CWindows_CDevices_CBluetooth_CGenericAttributeProfile_CIGattDeviceService** service);
    HRESULT (STDMETHODCALLTYPE* add_NameChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NameChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GattServicesChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GattServicesChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionStatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionStatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use GetGattServicesAsync instead of GattServices.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_get_GattServices(This, value) \
    ((This)->lpVtbl->get_GattServices(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_get_ConnectionStatus(This, value) \
    ((This)->lpVtbl->get_ConnectionStatus(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_get_BluetoothAddress(This, value) \
    ((This)->lpVtbl->get_BluetoothAddress(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("Use GetGattServicesForUuidAsync instead of GetGattService.  For more information, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_GetGattService(This, serviceUuid, service) \
    ((This)->lpVtbl->GetGattService(This, serviceUuid, service))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_add_NameChanged(This, handler, token) \
    ((This)->lpVtbl->add_NameChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_remove_NameChanged(This, token) \
    ((This)->lpVtbl->remove_NameChanged(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_add_GattServicesChanged(This, handler, token) \
    ((This)->lpVtbl->add_GattServicesChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_remove_GattServicesChanged(This, token) \
    ((This)->lpVtbl->remove_GattServicesChanged(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_add_ConnectionStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_remove_ConnectionStatusChanged(This, token) \
    ((This)->lpVtbl->remove_ConnectionStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice2[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_Appearance)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance** value);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddressType)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_get_Appearance(This, value) \
    ((This)->lpVtbl->get_Appearance(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_get_BluetoothAddressType(This, value) \
    ((This)->lpVtbl->get_BluetoothAddressType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice3[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAccessInformation)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* GetGattServicesAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetGattServicesWithCacheModeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetGattServicesForUuidAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        GUID serviceUuid,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetGattServicesForUuidWithCacheModeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3* This,
        GUID serviceUuid,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothCacheMode cacheMode,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CGenericAttributeProfile__CGattDeviceServicesResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_get_DeviceAccessInformation(This, value) \
    ((This)->lpVtbl->get_DeviceAccessInformation(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetGattServicesAsync(This, operation) \
    ((This)->lpVtbl->GetGattServicesAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetGattServicesWithCacheModeAsync(This, cacheMode, operation) \
    ((This)->lpVtbl->GetGattServicesWithCacheModeAsync(This, cacheMode, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetGattServicesForUuidAsync(This, serviceUuid, operation) \
    ((This)->lpVtbl->GetGattServicesForUuidAsync(This, serviceUuid, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_GetGattServicesForUuidWithCacheModeAsync(This, serviceUuid, cacheMode, operation) \
    ((This)->lpVtbl->GetGattServicesForUuidWithCacheModeAsync(This, serviceUuid, cacheMode, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice4[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice4";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothDeviceId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothDeviceId** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_get_BluetoothDeviceId(This, value) \
    ((This)->lpVtbl->get_BluetoothDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice5[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice5";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WasSecureConnectionUsedForPairing)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_get_WasSecureConnectionUsedForPairing(This, value) \
    ((This)->lpVtbl->get_WasSecureConnectionUsedForPairing(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDevice6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDevice6[] = L"Windows.Devices.Bluetooth.IBluetoothLEDevice6";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetConnectionParameters)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionParameters** result);
    HRESULT (STDMETHODCALLTYPE* GetConnectionPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEConnectionPhy** result);
    HRESULT (STDMETHODCALLTYPE* RequestPreferredConnectionParameters)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* preferredConnectionParameters,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest** result);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionParametersChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionParametersChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionPhyChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CBluetoothLEDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionPhyChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_GetConnectionParameters(This, result) \
    ((This)->lpVtbl->GetConnectionParameters(This, result))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_GetConnectionPhy(This, result) \
    ((This)->lpVtbl->GetConnectionPhy(This, result))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_RequestPreferredConnectionParameters(This, preferredConnectionParameters, result) \
    ((This)->lpVtbl->RequestPreferredConnectionParameters(This, preferredConnectionParameters, result))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_add_ConnectionParametersChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionParametersChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_remove_ConnectionParametersChanged(This, token) \
    ((This)->lpVtbl->remove_ConnectionParametersChanged(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_add_ConnectionPhyChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionPhyChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_remove_ConnectionPhyChanged(This, token) \
    ((This)->lpVtbl->remove_ConnectionPhyChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDevice6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDeviceStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation);
    HRESULT (STDMETHODCALLTYPE* FromBluetoothAddressAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        UINT64 bluetoothAddress,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_FromBluetoothAddressAsync(This, bluetoothAddress, operation) \
    ((This)->lpVtbl->FromBluetoothAddressAsync(This, bluetoothAddress, operation))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEDeviceStatics2[] = L"Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromPairingState)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        boolean pairingState,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromConnectionStatus)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothConnectionStatus connectionStatus,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromDeviceName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        HSTRING deviceName,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromBluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        UINT64 bluetoothAddress,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        UINT64 bluetoothAddress,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType bluetoothAddressType,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromAppearance)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEAppearance* appearance,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* FromBluetoothAddressWithBluetoothAddressTypeAsync)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2* This,
        UINT64 bluetoothAddress,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType bluetoothAddressType,
        __FIAsyncOperation_1_Windows__CDevices__CBluetooth__CBluetoothLEDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromPairingState(This, pairingState, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromPairingState(This, pairingState, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromConnectionStatus(This, connectionStatus, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromConnectionStatus(This, connectionStatus, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromDeviceName(This, deviceName, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromDeviceName(This, deviceName, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromBluetoothAddress(This, bluetoothAddress, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromBluetoothAddress(This, bluetoothAddress, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(This, bluetoothAddress, bluetoothAddressType, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(This, bluetoothAddress, bluetoothAddressType, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_GetDeviceSelectorFromAppearance(This, appearance, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelectorFromAppearance(This, appearance, deviceSelector))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_FromBluetoothAddressWithBluetoothAddressTypeAsync(This, bluetoothAddress, bluetoothAddressType, operation) \
    ((This)->lpVtbl->FromBluetoothAddressWithBluetoothAddressTypeAsync(This, bluetoothAddress, bluetoothAddressType, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParameters[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LinkTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionLatency)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_MinConnectionInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxConnectionInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_get_LinkTimeout(This, value) \
    ((This)->lpVtbl->get_LinkTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_get_ConnectionLatency(This, value) \
    ((This)->lpVtbl->get_ConnectionLatency(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_get_MinConnectionInterval(This, value) \
    ((This)->lpVtbl->get_MinConnectionInterval(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_get_MaxConnectionInterval(This, value) \
    ((This)->lpVtbl->get_MaxConnectionInterval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParametersRequest[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothLEPreferredConnectionParametersRequestStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequestVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothLEPreferredConnectionParametersStatics[] = L"Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Balanced)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters** value);
    HRESULT (STDMETHODCALLTYPE* get_ThroughputOptimized)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters** value);
    HRESULT (STDMETHODCALLTYPE* get_PowerOptimized)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParameters** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_get_Balanced(This, value) \
    ((This)->lpVtbl->get_Balanced(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_get_ThroughputOptimized(This, value) \
    ((This)->lpVtbl->get_ThroughputOptimized(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_get_PowerOptimized(This, value) \
    ((This)->lpVtbl->get_PowerOptimized(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothLEPreferredConnectionParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothSignalStrengthFilter[] = L"Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InRangeThresholdInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_short** value);
    HRESULT (STDMETHODCALLTYPE* put_InRangeThresholdInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_short* value);
    HRESULT (STDMETHODCALLTYPE* get_OutOfRangeThresholdInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_short** value);
    HRESULT (STDMETHODCALLTYPE* put_OutOfRangeThresholdInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_short* value);
    HRESULT (STDMETHODCALLTYPE* get_OutOfRangeTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_OutOfRangeTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SamplingInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_SamplingInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilterVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_get_InRangeThresholdInDBm(This, value) \
    ((This)->lpVtbl->get_InRangeThresholdInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_put_InRangeThresholdInDBm(This, value) \
    ((This)->lpVtbl->put_InRangeThresholdInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_get_OutOfRangeThresholdInDBm(This, value) \
    ((This)->lpVtbl->get_OutOfRangeThresholdInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_put_OutOfRangeThresholdInDBm(This, value) \
    ((This)->lpVtbl->put_OutOfRangeThresholdInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_get_OutOfRangeTimeout(This, value) \
    ((This)->lpVtbl->get_OutOfRangeTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_put_OutOfRangeTimeout(This, value) \
    ((This)->lpVtbl->put_OutOfRangeTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_get_SamplingInterval(This, value) \
    ((This)->lpVtbl->get_SamplingInterval(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_put_SamplingInterval(This, value) \
    ((This)->lpVtbl->put_SamplingInterval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.BluetoothUuidHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_IBluetoothUuidHelperStatics[] = L"Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromShortId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        UINT32 shortId,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* TryGetShortId)(__x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics* This,
        GUID uuid,
        __FIReference_1_UINT32** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_FromShortId(This, shortId, result) \
    ((This)->lpVtbl->FromShortId(This, shortId, result))

#define __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_TryGetShortId(This, uuid, result) \
    ((This)->lpVtbl->TryGetShortId(This, uuid, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothUuidHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothAdapterStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothAdapter ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothAdapter2
 *    Windows.Devices.Bluetooth.IBluetoothAdapter3
 *    Windows.Devices.Bluetooth.IBluetoothAdapter4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothAdapter[] = L"Windows.Devices.Bluetooth.BluetoothAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothClassOfDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothClassOfDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothClassOfDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothClassOfDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothClassOfDevice[] = L"Windows.Devices.Bluetooth.BluetoothClassOfDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothDevice ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothDevice2
 *    Windows.Devices.Bluetooth.IBluetoothDevice3
 *    Windows.Devices.Bluetooth.IBluetoothDevice4
 *    Windows.Devices.Bluetooth.IBluetoothDevice5
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothDevice[] = L"Windows.Devices.Bluetooth.BluetoothDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothDeviceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothDeviceId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDeviceId_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothDeviceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothDeviceId[] = L"Windows.Devices.Bluetooth.BluetoothDeviceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEAppearance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearance_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearance[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearanceCategories[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEAppearanceSubcategories[] = L"Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionParameters[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionPhy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionPhy[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEConnectionPhyInfo[] = L"Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice ** Default Interface **
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice2
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice3
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice4
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice5
 *    Windows.Devices.Bluetooth.IBluetoothLEDevice6
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEDevice[] = L"Windows.Devices.Bluetooth.BluetoothLEDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParameters[] = L"Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothLEPreferredConnectionParametersRequest[] = L"Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothSignalStrengthFilter[] = L"Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.BluetoothUuidHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothUuidHelper_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_BluetoothUuidHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_BluetoothUuidHelper[] = L"Windows.Devices.Bluetooth.BluetoothUuidHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ebluetooth_p_h__

#endif // __windows2Edevices2Ebluetooth_h__
