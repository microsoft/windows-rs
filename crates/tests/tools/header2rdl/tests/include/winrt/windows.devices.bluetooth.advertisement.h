
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
#ifndef __windows2Edevices2Ebluetooth2Eadvertisement_h__
#define __windows2Edevices2Ebluetooth2Eadvertisement_h__
#ifndef __windows2Edevices2Ebluetooth2Eadvertisement_p_h__
#define __windows2Edevices2Ebluetooth2Eadvertisement_p_h__


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
#include "Windows.Devices.Bluetooth.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisement;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementBytePattern;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementBytePatternFactory;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePatternFactory

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementDataSection;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementDataSectionFactory;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSectionFactory

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementDataTypesStatics;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataTypesStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementFilter;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementFilter

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisher;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisher

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisher2;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisher2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisher3;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisher3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisherFactory;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisherFactory

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisherStatusChangedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisherStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementReceivedEventArgs2;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementReceivedEventArgs3;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementScanParameters;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParameters

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementScanParametersStatics;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParametersStatics

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementWatcher;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementWatcher2;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher2

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementWatcher3;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3 ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher3

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementWatcherFactory;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcherFactory

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEAdvertisementWatcherStoppedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcherStoppedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEManufacturerData;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    interface IBluetoothLEManufacturerDataFactory;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerDataFactory

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIIterator_1_GUID_USE
#define DEF___FIIterator_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3d64048-82b3-53c7-9285-b0be18368482"))
IIterator<GUID> : IIterator_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<GUID> __FIIterator_1_GUID_t;
#define __FIIterator_1_GUID ABI::Windows::Foundation::Collections::__FIIterator_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_GUID_USE */



#ifndef DEF___FIIterable_1_GUID_USE
#define DEF___FIIterable_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4ca3045-5dd7-54be-982e-d88d8ca0876e"))
IIterable<GUID> : IIterable_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<GUID> __FIIterable_1_GUID_t;
#define __FIIterable_1_GUID ABI::Windows::Foundation::Collections::__FIIterable_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_GUID_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementBytePattern;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b33e103a-1a61-5107-8813-c0e905c05486"))
IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1e3fadee-54ac-538b-8777-351afb78cb74"))
IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementDataSection;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af5c0e81-788b-52d4-82a2-1ed28c66a05e"))
IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db98b5d1-897e-59cc-b86a-7b8855ac98af"))
IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEManufacturerData;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#define DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("12f158dd-7016-5338-ac5c-7d5503d73274"))
IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t;
#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#define DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("834a4cac-bb8b-5f0f-9f28-4dbc98c17907"))
IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t;
#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_GUID_USE
#define DEF___FIVectorView_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9520e64b-15b2-52a6-98ed-3191fa6cf68a"))
IVectorView<GUID> : IVectorView_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<GUID> __FIVectorView_1_GUID_t;
#define __FIVectorView_1_GUID ABI::Windows::Foundation::Collections::__FIVectorView_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_GUID_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a7d9983a-a11f-572e-89fb-683ea429bcbc"))
IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c4f2b8ea-11a8-5109-9013-4047e12c72e8"))
IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#define DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("78ab070e-ad7e-5912-a4f1-7be33e4560af"))
IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t;
#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_GUID_USE
#define DEF___FIVector_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("482e676d-b913-5ec1-afa8-5f96922e94ae"))
IVector<GUID> : IVector_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<GUID> __FIVector_1_GUID_t;
#define __FIVector_1_GUID ABI::Windows::Foundation::Collections::__FIVector_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_GUID_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#define DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8dd461b7-9775-5e82-a0a6-6627abd0d010"))
IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementBytePattern*> __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t;
#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#define DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6f71ad2-e2cf-5d54-b6f1-90964ee5d4da"))
IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementDataSection*> __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t;
#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#define DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("52d75b45-1d24-5eeb-babb-65effae45e46"))
IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEManufacturerData*> __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t;
#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_USE */

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
                    typedef enum BluetoothLEAdvertisementFlags : unsigned int BluetoothLEAdvertisementFlags;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_USE
#define DEF___FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("91c0ba96-9e69-5b82-bf1d-83ab2a509c53"))
IReference<enum ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFlags> : IReference_impl<enum ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFlags>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFlags> __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_t;
#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags ABI::Windows::Foundation::__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementPublisher;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementPublisherStatusChangedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c2ffa4f1-5893-54a8-bd94-aa1198b05d07"))
ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatusChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisher*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatusChangedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisherStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher, Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatusChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementWatcher;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("90eb4eca-d465-5ea0-a61c-033c8c5ecef2"))
ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher, Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementWatcherStoppedEventArgs;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9936a4db-dc99-55c3-9e9b-bf4854bd9eab"))
ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcherStoppedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcherStoppedEventArgs*, ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcherStoppedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher, Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcher*, ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcherStoppedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
                typedef enum BluetoothError : int BluetoothError;
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
                namespace Advertisement {
                    typedef enum BluetoothLEAdvertisementPhyType : int BluetoothLEAdvertisementPhyType;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

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
                namespace Advertisement {
                    typedef enum BluetoothLEAdvertisementType : int BluetoothLEAdvertisementType;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    typedef enum BluetoothLEAdvertisementWatcherStatus : int BluetoothLEAdvertisementWatcherStatus;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    typedef enum BluetoothLEScanningMode : int BluetoothLEScanningMode;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisement;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementFilter;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    class BluetoothLEAdvertisementScanParameters;
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEAdvertisementFlags : unsigned int
                    {
                        BluetoothLEAdvertisementFlags_None = 0,
                        BluetoothLEAdvertisementFlags_LimitedDiscoverableMode = 0x1,
                        BluetoothLEAdvertisementFlags_GeneralDiscoverableMode = 0x2,
                        BluetoothLEAdvertisementFlags_ClassicNotSupported = 0x4,
                        BluetoothLEAdvertisementFlags_DualModeControllerCapable = 0x8,
                        BluetoothLEAdvertisementFlags_DualModeHostCapable = 0x10,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(BluetoothLEAdvertisementFlags)
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPhyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEAdvertisementPhyType : int
                    {
                        BluetoothLEAdvertisementPhyType_Unspecified = 0,
                        BluetoothLEAdvertisementPhyType_Uncoded1MPhy = 1,
                        BluetoothLEAdvertisementPhyType_Uncoded2MPhy = 2,
                        BluetoothLEAdvertisementPhyType_CodedPhy = 3,
                    };
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEAdvertisementPublisherStatus : int
                    {
                        BluetoothLEAdvertisementPublisherStatus_Created = 0,
                        BluetoothLEAdvertisementPublisherStatus_Waiting = 1,
                        BluetoothLEAdvertisementPublisherStatus_Started = 2,
                        BluetoothLEAdvertisementPublisherStatus_Stopping = 3,
                        BluetoothLEAdvertisementPublisherStatus_Stopped = 4,
                        BluetoothLEAdvertisementPublisherStatus_Aborted = 5,
                    };
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEAdvertisementType : int
                    {
                        BluetoothLEAdvertisementType_ConnectableUndirected = 0,
                        BluetoothLEAdvertisementType_ConnectableDirected = 1,
                        BluetoothLEAdvertisementType_ScannableUndirected = 2,
                        BluetoothLEAdvertisementType_NonConnectableUndirected = 3,
                        BluetoothLEAdvertisementType_ScanResponse = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        BluetoothLEAdvertisementType_Extended = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    };
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEAdvertisementWatcherStatus : int
                    {
                        BluetoothLEAdvertisementWatcherStatus_Created = 0,
                        BluetoothLEAdvertisementWatcherStatus_Started = 1,
                        BluetoothLEAdvertisementWatcherStatus_Stopping = 2,
                        BluetoothLEAdvertisementWatcherStatus_Stopped = 3,
                        BluetoothLEAdvertisementWatcherStatus_Aborted = 4,
                    };
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    enum BluetoothLEScanningMode : int
                    {
                        BluetoothLEScanningMode_Passive = 0,
                        BluetoothLEScanningMode_Active = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        BluetoothLEScanningMode_None = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    };
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisement[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("066fb2b7-33d1-4e7d-8367-cf81d0f79653")
                    IBluetoothLEAdvertisement : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Flags(
                            __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Flags(
                            __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_LocalName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceUuids(
                            __FIVector_1_GUID** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ManufacturerData(
                            __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DataSections(
                            __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetManufacturerDataByCompanyId(
                            UINT16 companyId,
                            __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** dataList
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSectionsByType(
                            BYTE type,
                            __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** sectionList
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisement = __uuidof(IBluetoothLEAdvertisement);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementBytePattern[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("fbfad7f2-b9c5-4a08-bc51-502f8ef68a79")
                    IBluetoothLEAdvertisementBytePattern : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DataType(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DataType(
                            BYTE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Offset(
                            INT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Offset(
                            INT16 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementBytePattern = __uuidof(IBluetoothLEAdvertisementBytePattern);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementBytePatternFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("c2e24d73-fd5c-4ec3-be2a-9ca6fa11b7bd")
                    IBluetoothLEAdvertisementBytePatternFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            BYTE dataType,
                            INT16 offset,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementBytePattern** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementBytePatternFactory = __uuidof(IBluetoothLEAdvertisementBytePatternFactory);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataSection[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("d7213314-3a43-40f9-b6f0-92bfefc34ae3")
                    IBluetoothLEAdvertisementDataSection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DataType(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DataType(
                            BYTE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementDataSection = __uuidof(IBluetoothLEAdvertisementDataSection);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataSectionFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("e7a40942-a845-4045-bf7e-3e9971db8a6b")
                    IBluetoothLEAdvertisementDataSectionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            BYTE dataType,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementDataSection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementDataSectionFactory = __uuidof(IBluetoothLEAdvertisementDataSectionFactory);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataTypesStatics[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("3bb6472f-0606-434b-a76e-74159f0684d3")
                    IBluetoothLEAdvertisementDataTypesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Flags(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncompleteService16BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompleteService16BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncompleteService32BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompleteService32BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncompleteService128BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompleteService128BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ShortenedLocalName(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompleteLocalName(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TxPowerLevel(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PeripheralConnectionIntervalRange(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceSolicitation16BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceSolicitation32BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceSolicitation128BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceData16BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceData32BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceData128BitUuids(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PublicTargetAddress(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RandomTargetAddress(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Appearance(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdvertisingInterval(
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ManufacturerSpecificData(
                            BYTE* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementDataTypesStatics = __uuidof(IBluetoothLEAdvertisementDataTypesStatics);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementFilter[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("131eb0d3-d04e-47b1-837e-49405bf6f80f")
                    IBluetoothLEAdvertisementFilter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Advertisement(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Advertisement(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BytePatterns(
                            __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementFilter = __uuidof(IBluetoothLEAdvertisementFilter);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("cde820f9-d9fa-43d6-a264-ddd8b7da8b78")
                    IBluetoothLEAdvertisementPublisher : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Advertisement(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisher = __uuidof(IBluetoothLEAdvertisementPublisher);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("fbdb545e-56f1-510f-a434-217fbd9e7bd2")
                    IBluetoothLEAdvertisementPublisher2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredTransmitPowerLevelInDBm(
                            __FIReference_1_short** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PreferredTransmitPowerLevelInDBm(
                            __FIReference_1_short* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UseExtendedAdvertisement(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseExtendedAdvertisement(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAnonymous(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsAnonymous(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncludeTransmitPowerLevel(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncludeTransmitPowerLevel(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisher2 = __uuidof(IBluetoothLEAdvertisementPublisher2);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("1cff3902-61ec-5776-ab86-9b41f94b1e66")
                    IBluetoothLEAdvertisementPublisher3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrimaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PrimaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecondaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SecondaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisher3 = __uuidof(IBluetoothLEAdvertisementPublisher3);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("5c5f065e-b863-4981-a1af-1c544d8b0c0d")
                    IBluetoothLEAdvertisementPublisherFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement* advertisement,
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementPublisher** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisherFactory = __uuidof(IBluetoothLEAdvertisementPublisherFactory);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("09c2bd9f-2dff-4b23-86ee-0d14fb94aeae")
                    IBluetoothLEAdvertisementPublisherStatusChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPublisherStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs = __uuidof(IBluetoothLEAdvertisementPublisherStatusChangedEventArgs);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("8f62790e-dc88-5c8b-b34e-10b321850f88")
                    IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SelectedTransmitPowerLevelInDBm(
                            __FIReference_1_short** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 = __uuidof(IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("27987ddf-e596-41be-8d43-9e6731d4a913")
                    IBluetoothLEAdvertisementReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RawSignalStrengthInDBm(
                            INT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddress(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdvertisementType(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Advertisement(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisement** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementReceivedEventArgs = __uuidof(IBluetoothLEAdvertisementReceivedEventArgs);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("12d9c87b-0399-5f0e-a348-53b02b6b162e")
                    IBluetoothLEAdvertisementReceivedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BluetoothAddressType(
                            ABI::Windows::Devices::Bluetooth::BluetoothAddressType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TransmitPowerLevelInDBm(
                            __FIReference_1_short** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAnonymous(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsConnectable(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsScannable(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDirected(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsScanResponse(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementReceivedEventArgs2 = __uuidof(IBluetoothLEAdvertisementReceivedEventArgs2);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("8d204b54-ff86-5d84-a25a-137dccd96f7a")
                    IBluetoothLEAdvertisementReceivedEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrimaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecondaryPhy(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementPhyType* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementReceivedEventArgs3 = __uuidof(IBluetoothLEAdvertisementReceivedEventArgs3);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementScanParameters[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("94f91413-63d9-53bd-af4c-e6b1a6514595")
                    IBluetoothLEAdvertisementScanParameters : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ScanWindow(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ScanInterval(
                            UINT16* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementScanParameters = __uuidof(IBluetoothLEAdvertisementScanParameters);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementScanParametersStatics[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("548e39cd-3c9e-5f8d-b5e1-adebed5c357c")
                    IBluetoothLEAdvertisementScanParametersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CoexistenceOptimized(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParameters** parameters
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LowLatency(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParameters** parameters
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementScanParametersStatics = __uuidof(IBluetoothLEAdvertisementScanParametersStatics);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("a6ac336f-f3d3-4297-8d6c-c81ea6623f40")
                    IBluetoothLEAdvertisementWatcher : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MinSamplingInterval(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxSamplingInterval(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MinOutOfRangeTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxOutOfRangeTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementWatcherStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ScanningMode(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEScanningMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ScanningMode(
                            ABI::Windows::Devices::Bluetooth::Advertisement::BluetoothLEScanningMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SignalStrengthFilter(
                            ABI::Windows::Devices::Bluetooth::IBluetoothSignalStrengthFilter** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SignalStrengthFilter(
                            ABI::Windows::Devices::Bluetooth::IBluetoothSignalStrengthFilter* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdvertisementFilter(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementFilter** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AdvertisementFilter(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementFilter* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Received(
                            __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Received(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                            __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcher = __uuidof(IBluetoothLEAdvertisementWatcher);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("01bf26bc-b164-5805-90a3-e8a7997ff225")
                    IBluetoothLEAdvertisementWatcher2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllowExtendedAdvertisements(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AllowExtendedAdvertisements(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcher2 = __uuidof(IBluetoothLEAdvertisementWatcher2);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("14d980be-4002-5dbe-8519-ffca6ca389f0")
                    IBluetoothLEAdvertisementWatcher3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UseUncoded1MPhy(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseUncoded1MPhy(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UseCodedPhy(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseCodedPhy(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ScanParameters(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParameters** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ScanParameters(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementScanParameters* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UseHardwareFilter(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseHardwareFilter(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcher3 = __uuidof(IBluetoothLEAdvertisementWatcher3);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcherFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("9aaf2d56-39ac-453e-b32a-85c657e017f1")
                    IBluetoothLEAdvertisementWatcherFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementFilter* advertisementFilter,
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEAdvertisementWatcher** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcherFactory = __uuidof(IBluetoothLEAdvertisementWatcherFactory);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcherStoppedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("dd40f84d-e7b9-43e3-9c04-0685d085fd8c")
                    IBluetoothLEAdvertisementWatcherStoppedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Error(
                            ABI::Windows::Devices::Bluetooth::BluetoothError* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEAdvertisementWatcherStoppedEventArgs = __uuidof(IBluetoothLEAdvertisementWatcherStoppedEventArgs);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEManufacturerData[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("912dba18-6963-4533-b061-4694dafb34e5")
                    IBluetoothLEManufacturerData : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CompanyId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CompanyId(
                            UINT16 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEManufacturerData = __uuidof(IBluetoothLEManufacturerData);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEManufacturerDataFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Bluetooth {
                namespace Advertisement {
                    MIDL_INTERFACE("c09b39f8-319a-441e-8de5-66a81e877a6c")
                    IBluetoothLEManufacturerDataFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            UINT16 companyId,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Devices::Bluetooth::Advertisement::IBluetoothLEManufacturerData** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBluetoothLEManufacturerDataFactory = __uuidof(IBluetoothLEManufacturerDataFactory);
                } /* Advertisement */
            } /* Bluetooth */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3 __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIIterator_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterator_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterator_1_GUID __FIIterator_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_GUID;

typedef struct __FIIterator_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_GUID* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_GUID* This,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_GUIDVtbl;

interface __FIIterator_1_GUID
{
    CONST_VTBL struct __FIIterator_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_GUID_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_GUID_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_GUID_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_GUID_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_GUID_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterable_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterable_1_GUID __FIIterable_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_GUID;

typedef struct __FIIterable_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_GUID* This,
        __FIIterator_1_GUID** result);

    END_INTERFACE
} __FIIterable_1_GUIDVtbl;

interface __FIIterable_1_GUID
{
    CONST_VTBL struct __FIIterable_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_GUID_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_GUID_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

typedef struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl;

interface __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

typedef struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __FIIterator_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl;

interface __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_GUID_INTERFACE_DEFINED__)
#define ____FIVectorView_1_GUID_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_GUID __FIVectorView_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_GUID;

typedef struct __FIVectorView_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_GUID* This,
        UINT32 index,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_GUID* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_GUID* This,
        GUID value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_GUID* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_GUIDVtbl;

interface __FIVectorView_1_GUID
{
    CONST_VTBL struct __FIVectorView_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_GUID_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_GUID_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_GUID_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_GUID_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_GUID_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

typedef struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl;

interface __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_GUID_INTERFACE_DEFINED__)
#define ____FIVector_1_GUID_INTERFACE_DEFINED__

typedef interface __FIVector_1_GUID __FIVector_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_GUID;

typedef struct __FIVector_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_GUID* This,
        UINT32 index,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_GUID* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_GUID* This,
        __FIVectorView_1_GUID** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_GUID* This,
        GUID value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_GUID* This,
        UINT32 index,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_GUID* This,
        UINT32 index,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_GUID* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_GUID* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_GUID* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_GUID* This,
        UINT32 itemsLength,
        GUID* items);

    END_INTERFACE
} __FIVector_1_GUIDVtbl;

interface __FIVector_1_GUID
{
    CONST_VTBL struct __FIVector_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_GUID_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_GUID_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_GUID_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_GUID_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_GUID_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_GUID_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_GUID_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_GUID_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_GUID_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_GUID_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_GUID_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_GUID_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_GUID_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern;

typedef struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl;

interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePatternVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection;

typedef struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl;

interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData;

typedef struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl;

interface __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData_INTERFACE_DEFINED__
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

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementFlags __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementFlags;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags;

typedef struct __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlagsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementFlags* result);

    END_INTERFACE
} __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlagsVtbl;

interface __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags
{
    CONST_VTBL struct __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlagsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* sender,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* sender,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* sender,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError;

#ifndef ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter;

#endif // ____x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementType __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementType;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementWatcherStatus __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementWatcherStatus;

typedef enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEScanningMode __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEScanningMode;

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementFlags
{
    BluetoothLEAdvertisementFlags_None = 0,
    BluetoothLEAdvertisementFlags_LimitedDiscoverableMode = 0x1,
    BluetoothLEAdvertisementFlags_GeneralDiscoverableMode = 0x2,
    BluetoothLEAdvertisementFlags_ClassicNotSupported = 0x4,
    BluetoothLEAdvertisementFlags_DualModeControllerCapable = 0x8,
    BluetoothLEAdvertisementFlags_DualModeHostCapable = 0x10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPhyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType
{
    BluetoothLEAdvertisementPhyType_Unspecified = 0,
    BluetoothLEAdvertisementPhyType_Uncoded1MPhy = 1,
    BluetoothLEAdvertisementPhyType_Uncoded2MPhy = 2,
    BluetoothLEAdvertisementPhyType_CodedPhy = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus
{
    BluetoothLEAdvertisementPublisherStatus_Created = 0,
    BluetoothLEAdvertisementPublisherStatus_Waiting = 1,
    BluetoothLEAdvertisementPublisherStatus_Started = 2,
    BluetoothLEAdvertisementPublisherStatus_Stopping = 3,
    BluetoothLEAdvertisementPublisherStatus_Stopped = 4,
    BluetoothLEAdvertisementPublisherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementType
{
    BluetoothLEAdvertisementType_ConnectableUndirected = 0,
    BluetoothLEAdvertisementType_ConnectableDirected = 1,
    BluetoothLEAdvertisementType_ScannableUndirected = 2,
    BluetoothLEAdvertisementType_NonConnectableUndirected = 3,
    BluetoothLEAdvertisementType_ScanResponse = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    BluetoothLEAdvertisementType_Extended = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementWatcherStatus
{
    BluetoothLEAdvertisementWatcherStatus_Created = 0,
    BluetoothLEAdvertisementWatcherStatus_Started = 1,
    BluetoothLEAdvertisementWatcherStatus_Stopping = 2,
    BluetoothLEAdvertisementWatcherStatus_Stopped = 3,
    BluetoothLEAdvertisementWatcherStatus_Aborted = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEScanningMode
{
    BluetoothLEScanningMode_Passive = 0,
    BluetoothLEScanningMode_Active = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    BluetoothLEScanningMode_None = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisement[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Flags)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags** value);
    HRESULT (STDMETHODCALLTYPE* put_Flags)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        __FIReference_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementFlags* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_LocalName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        __FIVector_1_GUID** value);
    HRESULT (STDMETHODCALLTYPE* get_ManufacturerData)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** value);
    HRESULT (STDMETHODCALLTYPE* get_DataSections)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** value);
    HRESULT (STDMETHODCALLTYPE* GetManufacturerDataByCompanyId)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        UINT16 companyId,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEManufacturerData** dataList);
    HRESULT (STDMETHODCALLTYPE* GetSectionsByType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* This,
        BYTE type,
        __FIVectorView_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementDataSection** sectionList);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_get_Flags(This, value) \
    ((This)->lpVtbl->get_Flags(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_put_Flags(This, value) \
    ((This)->lpVtbl->put_Flags(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_get_LocalName(This, value) \
    ((This)->lpVtbl->get_LocalName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_put_LocalName(This, value) \
    ((This)->lpVtbl->put_LocalName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_get_ServiceUuids(This, value) \
    ((This)->lpVtbl->get_ServiceUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_get_ManufacturerData(This, value) \
    ((This)->lpVtbl->get_ManufacturerData(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_get_DataSections(This, value) \
    ((This)->lpVtbl->get_DataSections(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_GetManufacturerDataByCompanyId(This, companyId, dataList) \
    ((This)->lpVtbl->GetManufacturerDataByCompanyId(This, companyId, dataList))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_GetSectionsByType(This, type, sectionList) \
    ((This)->lpVtbl->GetSectionsByType(This, type, sectionList))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementBytePattern[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_DataType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_Offset)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* put_Offset)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        INT16 value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_get_DataType(This, value) \
    ((This)->lpVtbl->get_DataType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_put_DataType(This, value) \
    ((This)->lpVtbl->put_DataType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_get_Offset(This, value) \
    ((This)->lpVtbl->get_Offset(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_put_Offset(This, value) \
    ((This)->lpVtbl->put_Offset(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementBytePatternFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory* This,
        BYTE dataType,
        INT16 offset,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePattern** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_Create(This, dataType, offset, data, value) \
    ((This)->lpVtbl->Create(This, dataType, offset, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementBytePatternFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataSection[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_DataType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_get_DataType(This, value) \
    ((This)->lpVtbl->get_DataType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_put_DataType(This, value) \
    ((This)->lpVtbl->put_DataType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataSectionFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory* This,
        BYTE dataType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_Create(This, dataType, data, value) \
    ((This)->lpVtbl->Create(This, dataType, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataSectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementDataTypesStatics[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Flags)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_IncompleteService16BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_CompleteService16BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_IncompleteService32BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_CompleteService32BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_IncompleteService128BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_CompleteService128BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ShortenedLocalName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_CompleteLocalName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_TxPowerLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_PeripheralConnectionIntervalRange)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceSolicitation16BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceSolicitation32BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceSolicitation128BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceData16BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceData32BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceData128BitUuids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicTargetAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_RandomTargetAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_Appearance)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisingInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ManufacturerSpecificData)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics* This,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_Flags(This, value) \
    ((This)->lpVtbl->get_Flags(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_IncompleteService16BitUuids(This, value) \
    ((This)->lpVtbl->get_IncompleteService16BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_CompleteService16BitUuids(This, value) \
    ((This)->lpVtbl->get_CompleteService16BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_IncompleteService32BitUuids(This, value) \
    ((This)->lpVtbl->get_IncompleteService32BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_CompleteService32BitUuids(This, value) \
    ((This)->lpVtbl->get_CompleteService32BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_IncompleteService128BitUuids(This, value) \
    ((This)->lpVtbl->get_IncompleteService128BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_CompleteService128BitUuids(This, value) \
    ((This)->lpVtbl->get_CompleteService128BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ShortenedLocalName(This, value) \
    ((This)->lpVtbl->get_ShortenedLocalName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_CompleteLocalName(This, value) \
    ((This)->lpVtbl->get_CompleteLocalName(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_TxPowerLevel(This, value) \
    ((This)->lpVtbl->get_TxPowerLevel(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_PeripheralConnectionIntervalRange(This, value) \
    ((This)->lpVtbl->get_PeripheralConnectionIntervalRange(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceSolicitation16BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceSolicitation16BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceSolicitation32BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceSolicitation32BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceSolicitation128BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceSolicitation128BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceData16BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceData16BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceData32BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceData32BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ServiceData128BitUuids(This, value) \
    ((This)->lpVtbl->get_ServiceData128BitUuids(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_PublicTargetAddress(This, value) \
    ((This)->lpVtbl->get_PublicTargetAddress(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_RandomTargetAddress(This, value) \
    ((This)->lpVtbl->get_RandomTargetAddress(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_Appearance(This, value) \
    ((This)->lpVtbl->get_Appearance(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_AdvertisingInterval(This, value) \
    ((This)->lpVtbl->get_AdvertisingInterval(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_get_ManufacturerSpecificData(This, value) \
    ((This)->lpVtbl->get_ManufacturerSpecificData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementDataTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementFilter[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Advertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement** value);
    HRESULT (STDMETHODCALLTYPE* put_Advertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* value);
    HRESULT (STDMETHODCALLTYPE* get_BytePatterns)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* This,
        __FIVector_1_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementBytePattern** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilterVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_get_Advertisement(This, value) \
    ((This)->lpVtbl->get_Advertisement(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_put_Advertisement(This, value) \
    ((This)->lpVtbl->put_Advertisement(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_get_BytePatterns(This, value) \
    ((This)->lpVtbl->get_BytePatterns(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Advertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement** value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementPublisherStatusChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_get_Advertisement(This, value) \
    ((This)->lpVtbl->get_Advertisement(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PreferredTransmitPowerLevelInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        __FIReference_1_short** value);
    HRESULT (STDMETHODCALLTYPE* put_PreferredTransmitPowerLevelInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        __FIReference_1_short* value);
    HRESULT (STDMETHODCALLTYPE* get_UseExtendedAdvertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseExtendedAdvertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsAnonymous)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsAnonymous)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeTransmitPowerLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeTransmitPowerLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_get_PreferredTransmitPowerLevelInDBm(This, value) \
    ((This)->lpVtbl->get_PreferredTransmitPowerLevelInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_put_PreferredTransmitPowerLevelInDBm(This, value) \
    ((This)->lpVtbl->put_PreferredTransmitPowerLevelInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_get_UseExtendedAdvertisement(This, value) \
    ((This)->lpVtbl->get_UseExtendedAdvertisement(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_put_UseExtendedAdvertisement(This, value) \
    ((This)->lpVtbl->put_UseExtendedAdvertisement(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_get_IsAnonymous(This, value) \
    ((This)->lpVtbl->get_IsAnonymous(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_put_IsAnonymous(This, value) \
    ((This)->lpVtbl->put_IsAnonymous(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_get_IncludeTransmitPowerLevel(This, value) \
    ((This)->lpVtbl->get_IncludeTransmitPowerLevel(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_put_IncludeTransmitPowerLevel(This, value) \
    ((This)->lpVtbl->put_IncludeTransmitPowerLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisher3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType* value);
    HRESULT (STDMETHODCALLTYPE* put_PrimaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType value);
    HRESULT (STDMETHODCALLTYPE* get_SecondaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType* value);
    HRESULT (STDMETHODCALLTYPE* put_SecondaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_get_PrimaryPhy(This, value) \
    ((This)->lpVtbl->get_PrimaryPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_put_PrimaryPhy(This, value) \
    ((This)->lpVtbl->put_PrimaryPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_get_SecondaryPhy(This, value) \
    ((This)->lpVtbl->get_SecondaryPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_put_SecondaryPhy(This, value) \
    ((This)->lpVtbl->put_SecondaryPhy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement* advertisement,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisher** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_Create(This, advertisement, value) \
    ((This)->lpVtbl->Create(This, advertisement, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPublisherStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedTransmitPowerLevelInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2* This,
        __FIReference_1_short** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_get_SelectedTransmitPowerLevelInDBm(This, value) \
    ((This)->lpVtbl->get_SelectedTransmitPowerLevelInDBm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RawSignalStrengthInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddress)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisementType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementType* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Advertisement)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisement** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_get_RawSignalStrengthInDBm(This, value) \
    ((This)->lpVtbl->get_RawSignalStrengthInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_get_BluetoothAddress(This, value) \
    ((This)->lpVtbl->get_BluetoothAddress(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_get_AdvertisementType(This, value) \
    ((This)->lpVtbl->get_AdvertisementType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_get_Advertisement(This, value) \
    ((This)->lpVtbl->get_Advertisement(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BluetoothAddressType)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothAddressType* value);
    HRESULT (STDMETHODCALLTYPE* get_TransmitPowerLevelInDBm)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        __FIReference_1_short** value);
    HRESULT (STDMETHODCALLTYPE* get_IsAnonymous)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnectable)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsScannable)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDirected)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsScanResponse)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_BluetoothAddressType(This, value) \
    ((This)->lpVtbl->get_BluetoothAddressType(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_TransmitPowerLevelInDBm(This, value) \
    ((This)->lpVtbl->get_TransmitPowerLevelInDBm(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_IsAnonymous(This, value) \
    ((This)->lpVtbl->get_IsAnonymous(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_IsConnectable(This, value) \
    ((This)->lpVtbl->get_IsConnectable(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_IsScannable(This, value) \
    ((This)->lpVtbl->get_IsScannable(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_IsDirected(This, value) \
    ((This)->lpVtbl->get_IsDirected(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_get_IsScanResponse(This, value) \
    ((This)->lpVtbl->get_IsScanResponse(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementReceivedEventArgs3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType* value);
    HRESULT (STDMETHODCALLTYPE* get_SecondaryPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementPhyType* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_get_PrimaryPhy(This, value) \
    ((This)->lpVtbl->get_PrimaryPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_get_SecondaryPhy(This, value) \
    ((This)->lpVtbl->get_SecondaryPhy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementReceivedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementScanParameters[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ScanWindow)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ScanInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_get_ScanWindow(This, value) \
    ((This)->lpVtbl->get_ScanWindow(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_get_ScanInterval(This, value) \
    ((This)->lpVtbl->get_ScanInterval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementScanParametersStatics[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CoexistenceOptimized)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters** parameters);
    HRESULT (STDMETHODCALLTYPE* LowLatency)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters** parameters);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_CoexistenceOptimized(This, parameters) \
    ((This)->lpVtbl->CoexistenceOptimized(This, parameters))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_LowLatency(This, parameters) \
    ((This)->lpVtbl->LowLatency(This, parameters))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MinSamplingInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSamplingInterval)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_MinOutOfRangeTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxOutOfRangeTimeout)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEAdvertisementWatcherStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ScanningMode)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEScanningMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ScanningMode)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CBluetoothLEScanningMode value);
    HRESULT (STDMETHODCALLTYPE* get_SignalStrengthFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter** value);
    HRESULT (STDMETHODCALLTYPE* put_SignalStrengthFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CIBluetoothSignalStrengthFilter* value);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisementFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter** value);
    HRESULT (STDMETHODCALLTYPE* put_AdvertisementFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This);
    HRESULT (STDMETHODCALLTYPE* add_Received)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Received)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcher_Windows__CDevices__CBluetooth__CAdvertisement__CBluetoothLEAdvertisementWatcherStoppedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_MinSamplingInterval(This, value) \
    ((This)->lpVtbl->get_MinSamplingInterval(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_MaxSamplingInterval(This, value) \
    ((This)->lpVtbl->get_MaxSamplingInterval(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_MinOutOfRangeTimeout(This, value) \
    ((This)->lpVtbl->get_MinOutOfRangeTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_MaxOutOfRangeTimeout(This, value) \
    ((This)->lpVtbl->get_MaxOutOfRangeTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_ScanningMode(This, value) \
    ((This)->lpVtbl->get_ScanningMode(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_put_ScanningMode(This, value) \
    ((This)->lpVtbl->put_ScanningMode(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_SignalStrengthFilter(This, value) \
    ((This)->lpVtbl->get_SignalStrengthFilter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_put_SignalStrengthFilter(This, value) \
    ((This)->lpVtbl->put_SignalStrengthFilter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_get_AdvertisementFilter(This, value) \
    ((This)->lpVtbl->get_AdvertisementFilter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_put_AdvertisementFilter(This, value) \
    ((This)->lpVtbl->put_AdvertisementFilter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_add_Received(This, handler, token) \
    ((This)->lpVtbl->add_Received(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_remove_Received(This, token) \
    ((This)->lpVtbl->remove_Received(This, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher2[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowExtendedAdvertisements)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowExtendedAdvertisements)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_get_AllowExtendedAdvertisements(This, value) \
    ((This)->lpVtbl->get_AllowExtendedAdvertisements(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_put_AllowExtendedAdvertisements(This, value) \
    ((This)->lpVtbl->put_AllowExtendedAdvertisements(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcher3[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UseUncoded1MPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseUncoded1MPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_UseCodedPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseCodedPhy)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ScanParameters)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters** value);
    HRESULT (STDMETHODCALLTYPE* put_ScanParameters)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementScanParameters* value);
    HRESULT (STDMETHODCALLTYPE* get_UseHardwareFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseHardwareFilter)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3Vtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_get_UseUncoded1MPhy(This, value) \
    ((This)->lpVtbl->get_UseUncoded1MPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_put_UseUncoded1MPhy(This, value) \
    ((This)->lpVtbl->put_UseUncoded1MPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_get_UseCodedPhy(This, value) \
    ((This)->lpVtbl->get_UseCodedPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_put_UseCodedPhy(This, value) \
    ((This)->lpVtbl->put_UseCodedPhy(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_get_ScanParameters(This, value) \
    ((This)->lpVtbl->get_ScanParameters(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_put_ScanParameters(This, value) \
    ((This)->lpVtbl->put_ScanParameters(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_get_UseHardwareFilter(This, value) \
    ((This)->lpVtbl->get_UseHardwareFilter(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_put_UseHardwareFilter(This, value) \
    ((This)->lpVtbl->put_UseHardwareFilter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcherFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory* This,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementFilter* advertisementFilter,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcher** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_Create(This, advertisementFilter, value) \
    ((This)->lpVtbl->Create(This, advertisementFilter, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEAdvertisementWatcherStoppedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CBluetooth_CBluetoothError* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEAdvertisementWatcherStoppedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEManufacturerData[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CompanyId)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_CompanyId)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_get_CompanyId(This, value) \
    ((This)->lpVtbl->get_CompanyId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_put_CompanyId(This, value) \
    ((This)->lpVtbl->put_CompanyId(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Bluetooth_Advertisement_IBluetoothLEManufacturerDataFactory[] = L"Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory";
typedef struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory* This,
        UINT16 companyId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerData** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_Create(This, companyId, data, value) \
    ((This)->lpVtbl->Create(This, companyId, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBluetooth_CAdvertisement_CIBluetoothLEManufacturerDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisement[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementBytePattern[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataSection[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementDataTypes[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementFilter[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisher[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementPublisherStatusChangedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementReceivedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParametersStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementScanParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementScanParameters[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementScanParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher ** Default Interface **
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcher[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEAdvertisementWatcherStoppedEventArgs[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData_DEFINED
#define RUNTIMECLASS_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Bluetooth_Advertisement_BluetoothLEManufacturerData[] = L"Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
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
#endif // __windows2Edevices2Ebluetooth2Eadvertisement_p_h__

#endif // __windows2Edevices2Ebluetooth2Eadvertisement_h__
