
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
#ifndef __windows2Edevices2Eenumeration_h__
#define __windows2Edevices2Eenumeration_h__
#ifndef __windows2Edevices2Eenumeration_p_h__
#define __windows2Edevices2Eenumeration_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Background.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessChangedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs ABI::Windows::Devices::Enumeration::IDeviceAccessChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessChangedEventArgs2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2 ABI::Windows::Devices::Enumeration::IDeviceAccessChangedEventArgs2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessChangedEventArgs3;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3 ABI::Windows::Devices::Enumeration::IDeviceAccessChangedEventArgs3

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessInformation2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2 ABI::Windows::Devices::Enumeration::IDeviceAccessInformation2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceAccessInformationStatics;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics ABI::Windows::Devices::Enumeration::IDeviceAccessInformationStatics

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceConnectionChangeTriggerDetails;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails ABI::Windows::Devices::Enumeration::IDeviceConnectionChangeTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceDisconnectButtonClickedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs ABI::Windows::Devices::Enumeration::IDeviceDisconnectButtonClickedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceEnumerationSettings;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings ABI::Windows::Devices::Enumeration::IDeviceEnumerationSettings

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformation2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2 ABI::Windows::Devices::Enumeration::IDeviceInformation2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationCustomPairing;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing ABI::Windows::Devices::Enumeration::IDeviceInformationCustomPairing

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationCustomPairing2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2 ABI::Windows::Devices::Enumeration::IDeviceInformationCustomPairing2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationPairing;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing ABI::Windows::Devices::Enumeration::IDeviceInformationPairing

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationPairing2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2 ABI::Windows::Devices::Enumeration::IDeviceInformationPairing2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationPairingStatics;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics ABI::Windows::Devices::Enumeration::IDeviceInformationPairingStatics

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationPairingStatics2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2 ABI::Windows::Devices::Enumeration::IDeviceInformationPairingStatics2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationStatics;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics ABI::Windows::Devices::Enumeration::IDeviceInformationStatics

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationStatics2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2 ABI::Windows::Devices::Enumeration::IDeviceInformationStatics2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationStatics3;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3 ABI::Windows::Devices::Enumeration::IDeviceInformationStatics3

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationUpdate;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate ABI::Windows::Devices::Enumeration::IDeviceInformationUpdate

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformationUpdate2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2 ABI::Windows::Devices::Enumeration::IDeviceInformationUpdate2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingRequestedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs ABI::Windows::Devices::Enumeration::IDevicePairingRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingRequestedEventArgs2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2 ABI::Windows::Devices::Enumeration::IDevicePairingRequestedEventArgs2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingRequestedEventArgs3;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3 ABI::Windows::Devices::Enumeration::IDevicePairingRequestedEventArgs3

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingResult;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult ABI::Windows::Devices::Enumeration::IDevicePairingResult

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingSetMembersRequestedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs ABI::Windows::Devices::Enumeration::IDevicePairingSetMembersRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePairingSettings;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings ABI::Windows::Devices::Enumeration::IDevicePairingSettings

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePicker;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker ABI::Windows::Devices::Enumeration::IDevicePicker

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePickerAppearance;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance ABI::Windows::Devices::Enumeration::IDevicePickerAppearance

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePickerFilter;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter ABI::Windows::Devices::Enumeration::IDevicePickerFilter

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceSelectedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs ABI::Windows::Devices::Enumeration::IDeviceSelectedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceUnpairingResult;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult ABI::Windows::Devices::Enumeration::IDeviceUnpairingResult

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceWatcher;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher ABI::Windows::Devices::Enumeration::IDeviceWatcher

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceWatcher2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2 ABI::Windows::Devices::Enumeration::IDeviceWatcher2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceWatcherEvent;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent ABI::Windows::Devices::Enumeration::IDeviceWatcherEvent

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceWatcherTriggerDetails;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails ABI::Windows::Devices::Enumeration::IDeviceWatcherTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IEnclosureLocation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation ABI::Windows::Devices::Enumeration::IEnclosureLocation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IEnclosureLocation2;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2 ABI::Windows::Devices::Enumeration::IEnclosureLocation2

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07faa053-eb2f-5cba-b25b-d9d57be6715f"))
IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceInformation*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceInformation*> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb483df2-7bb6-5923-a28d-8342ec30046b"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceInformation*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceInformation*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformationCollection;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6f85d843-e8ab-5b46-85d7-327c58d18712"))
IIterator<ABI::Windows::Devices::Enumeration::DeviceInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Enumeration::DeviceInformation*> __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dd9f8a5d-ec98-5f4b-a3ea-9c8b5ad53c4b"))
IIterable<ABI::Windows::Devices::Enumeration::DeviceInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Enumeration::DeviceInformation*> __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e170688f-3495-5bf6-aab5-9cac17e0f10f"))
IVectorView<ABI::Windows::Devices::Enumeration::DeviceInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Enumeration::DeviceInformation*> __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("45180254-082e-5274-b2e7-ac0517f44d07"))
IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*, __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceInformationCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4a458732-527e-5c73-9a68-a73da370f782"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*, __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceInformationCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCollection*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePairingResult;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1002db74-8948-591e-815d-e40b667599a3"))
IAsyncOperation<ABI::Windows::Devices::Enumeration::DevicePairingResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePairingResult*, ABI::Windows::Devices::Enumeration::IDevicePairingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DevicePairingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Enumeration::DevicePairingResult*> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7ee0247f-5f57-5cb2-b40e-18b5a211d6c3"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DevicePairingResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePairingResult*, ABI::Windows::Devices::Enumeration::IDevicePairingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DevicePairingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DevicePairingResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceThumbnail;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

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

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bac083a3-3a19-5072-9d90-133323a049ba"))
IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceThumbnail*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceThumbnail*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceThumbnail*> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("86d455b2-d795-554c-9c31-bf6539349c19"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceThumbnail*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceThumbnail*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceThumbnail*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceUnpairingResult;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2bb4df3d-bd7e-5fe0-9020-56dc0d30b935"))
IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*, ABI::Windows::Devices::Enumeration::IDeviceUnpairingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceUnpairingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9bbe6eb9-db2d-5160-a20c-f0c265f20d8e"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*, ABI::Windows::Devices::Enumeration::IDeviceUnpairingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceUnpairingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Enumeration::DeviceUnpairingResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000


#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceClass : int DeviceClass;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#define DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c3807283-1416-593c-955c-0b4a286ff7bb"))
IIterator<enum ABI::Windows::Devices::Enumeration::DeviceClass> : IIterator_impl<enum ABI::Windows::Devices::Enumeration::DeviceClass>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Enumeration.DeviceClass>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Devices::Enumeration::DeviceClass> __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_t;
#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#define DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("47d4be05-58f1-522e-81c6-975eb4131bb9"))
IIterable<enum ABI::Windows::Devices::Enumeration::DeviceClass> : IIterable_impl<enum ABI::Windows::Devices::Enumeration::DeviceClass>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceClass>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Devices::Enumeration::DeviceClass> __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_t;
#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceWatcherEvent;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#define DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("74f7d6cc-9c20-5bb9-bace-b2ffa38687f9"))
IIterator<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*, ABI::Windows::Devices::Enumeration::IDeviceWatcherEvent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Enumeration.DeviceWatcherEvent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t;
#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#define DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b48fd051-eafa-523f-a66e-9d4151c5d522"))
IIterable<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*, ABI::Windows::Devices::Enumeration::IDeviceWatcherEvent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceWatcherEvent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t;
#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceWatcherEventKind : int DeviceWatcherEventKind;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE
#define DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb5ca9db-ccd6-5103-a93d-c925c908838d"))
IIterator<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind> : IIterator_impl<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Enumeration.DeviceWatcherEventKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind> __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_t;
#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE
#define DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f04365ab-d3f3-5f85-a7da-dc19cff73d86"))
IIterable<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind> : IIterable_impl<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceWatcherEventKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind> __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_t;
#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#define DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("01a80a97-bd87-5c8a-97fd-d449c98bdac6"))
IVectorView<enum ABI::Windows::Devices::Enumeration::DeviceClass> : IVectorView_impl<enum ABI::Windows::Devices::Enumeration::DeviceClass>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Enumeration.DeviceClass>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Devices::Enumeration::DeviceClass> __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_t;
#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#define DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8f994d37-8fab-51c6-a1e0-c93f68a20ef0"))
IVectorView<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*, ABI::Windows::Devices::Enumeration::IDeviceWatcherEvent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Enumeration.DeviceWatcherEvent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Enumeration::DeviceWatcherEvent*> __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t;
#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#define DEF___FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee662d37-b0eb-5729-9832-156fd2889d48"))
IVector<enum ABI::Windows::Devices::Enumeration::DeviceClass> : IVector_impl<enum ABI::Windows::Devices::Enumeration::DeviceClass>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Enumeration.DeviceClass>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Devices::Enumeration::DeviceClass> __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_t;
#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceAccessInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceAccessChangedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4c71d028-b793-5bce-ae59-fa77f45a40d8"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceAccessInformation*, ABI::Windows::Devices::Enumeration::DeviceAccessChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceAccessInformation*, ABI::Windows::Devices::Enumeration::IDeviceAccessInformation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceAccessChangedEventArgs*, ABI::Windows::Devices::Enumeration::IDeviceAccessChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceAccessInformation, Windows.Devices.Enumeration.DeviceAccessChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceAccessInformation*, ABI::Windows::Devices::Enumeration::DeviceAccessChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformationCustomPairing;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePairingRequestedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fa65231f-4178-5de1-b2cc-03e22d7702b4"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::DevicePairingRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::IDeviceInformationCustomPairing*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePairingRequestedEventArgs*, ABI::Windows::Devices::Enumeration::IDevicePairingRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceInformationCustomPairing, Windows.Devices.Enumeration.DevicePairingRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::DevicePairingRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePairingSetMembersRequestedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5cc01096-b025-5d93-8cb2-5c4dfa992e07"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::DevicePairingSetMembersRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::IDeviceInformationCustomPairing*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePairingSetMembersRequestedEventArgs*, ABI::Windows::Devices::Enumeration::IDevicePairingSetMembersRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceInformationCustomPairing, Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceInformationCustomPairing*, ABI::Windows::Devices::Enumeration::DevicePairingSetMembersRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePicker;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("62c6d98c-57ee-5bb8-a41c-958d20c3f3e8"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::IDevicePicker*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DevicePicker, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceDisconnectButtonClickedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("35dd0319-5723-506c-8896-1a28b82be798"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::DeviceDisconnectButtonClickedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::IDevicePicker*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceDisconnectButtonClickedEventArgs*, ABI::Windows::Devices::Enumeration::IDeviceDisconnectButtonClickedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DevicePicker, Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::DeviceDisconnectButtonClickedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceSelectedEventArgs;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("47e48c88-1c56-5b58-96a2-8e813d25077a"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::DeviceSelectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::IDevicePicker*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceSelectedEventArgs*, ABI::Windows::Devices::Enumeration::IDeviceSelectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DevicePicker, Windows.Devices.Enumeration.DeviceSelectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DevicePicker*, ABI::Windows::Devices::Enumeration::DeviceSelectedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceWatcher;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9234630f-1ff4-54f6-9e3f-ac20369b7725"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::IDeviceWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03c5a07b-990c-5d09-b0b8-5734eaa38222"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::DeviceInformation*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::IDeviceWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformation*, ABI::Windows::Devices::Enumeration::IDeviceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceWatcher, Windows.Devices.Enumeration.DeviceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::DeviceInformation*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformationUpdate;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("906f1254-79ad-54fc-93c4-cdb99b437899"))
ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::DeviceInformationUpdate*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::IDeviceWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Enumeration::DeviceInformationUpdate*, ABI::Windows::Devices::Enumeration::IDeviceInformationUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Enumeration.DeviceWatcher, Windows.Devices.Enumeration.DeviceInformationUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Enumeration::DeviceWatcher*, ABI::Windows::Devices::Enumeration::DeviceInformationUpdate*> __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_t;
#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                class DeviceWatcherTrigger;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IDeviceWatcherTrigger;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger ABI::Windows::ApplicationModel::Background::IDeviceWatcherTrigger

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IPasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential ABI::Windows::Security::Credentials::IPasswordCredential

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IContentTypeProvider;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider ABI::Windows::Storage::Streams::IContentTypeProvider

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceAccessStatus : int DeviceAccessStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceInformationKind : int DeviceInformationKind;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DevicePairingAddPairingSetMemberStatus : int DevicePairingAddPairingSetMemberStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DevicePairingKinds : unsigned int DevicePairingKinds;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DevicePairingProtectionLevel : int DevicePairingProtectionLevel;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DevicePairingResultStatus : int DevicePairingResultStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DevicePickerDisplayStatusOptions : unsigned int DevicePickerDisplayStatusOptions;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceUnpairingResultStatus : int DeviceUnpairingResultStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceWatcherStatus : int DeviceWatcherStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum Panel : int Panel;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformationPairing;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePickerAppearance;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePickerFilter;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class EnclosureLocation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceAccessStatus : int
                {
                    DeviceAccessStatus_Unspecified = 0,
                    DeviceAccessStatus_Allowed = 1,
                    DeviceAccessStatus_DeniedByUser = 2,
                    DeviceAccessStatus_DeniedBySystem = 3,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceClass : int
                {
                    DeviceClass_All = 0,
                    DeviceClass_AudioCapture = 1,
                    DeviceClass_AudioRender = 2,
                    DeviceClass_PortableStorageDevice = 3,
                    DeviceClass_VideoCapture = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DeviceClass_ImageScanner = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DeviceClass_Location = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceInformationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceInformationKind : int
                {
                    DeviceInformationKind_Unknown = 0,
                    DeviceInformationKind_DeviceInterface = 1,
                    DeviceInformationKind_DeviceContainer = 2,
                    DeviceInformationKind_Device = 3,
                    DeviceInformationKind_DeviceInterfaceClass = 4,
                    DeviceInformationKind_AssociationEndpoint = 5,
                    DeviceInformationKind_AssociationEndpointContainer = 6,
                    DeviceInformationKind_AssociationEndpointService = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    DeviceInformationKind_DevicePanel = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    DeviceInformationKind_AssociationEndpointProtocol = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingAddPairingSetMemberStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DevicePairingAddPairingSetMemberStatus : int
                {
                    DevicePairingAddPairingSetMemberStatus_AddedToSet = 0,
                    DevicePairingAddPairingSetMemberStatus_CouldNotBeAddedToSet = 1,
                    DevicePairingAddPairingSetMemberStatus_SetDiscoveryNotAttemptedByProtocol = 2,
                    DevicePairingAddPairingSetMemberStatus_SetDiscoveryCompletedByProtocol = 3,
                    DevicePairingAddPairingSetMemberStatus_SetDiscoveryPartiallyCompletedByProtocol = 4,
                    DevicePairingAddPairingSetMemberStatus_Failed = 5,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DevicePairingKinds : unsigned int
                {
                    DevicePairingKinds_None = 0,
                    DevicePairingKinds_ConfirmOnly = 0x1,
                    DevicePairingKinds_DisplayPin = 0x2,
                    DevicePairingKinds_ProvidePin = 0x4,
                    DevicePairingKinds_ConfirmPinMatch = 0x8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
                    DevicePairingKinds_ProvidePasswordCredential = 0x10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    DevicePairingKinds_ProvideAddress = 0x20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                };

                DEFINE_ENUM_FLAG_OPERATORS(DevicePairingKinds)
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DevicePairingProtectionLevel : int
                {
                    DevicePairingProtectionLevel_Default = 0,
                    DevicePairingProtectionLevel_None = 1,
                    DevicePairingProtectionLevel_Encryption = 2,
                    DevicePairingProtectionLevel_EncryptionAndAuthentication = 3,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DevicePairingResultStatus : int
                {
                    DevicePairingResultStatus_Paired = 0,
                    DevicePairingResultStatus_NotReadyToPair = 1,
                    DevicePairingResultStatus_NotPaired = 2,
                    DevicePairingResultStatus_AlreadyPaired = 3,
                    DevicePairingResultStatus_ConnectionRejected = 4,
                    DevicePairingResultStatus_TooManyConnections = 5,
                    DevicePairingResultStatus_HardwareFailure = 6,
                    DevicePairingResultStatus_AuthenticationTimeout = 7,
                    DevicePairingResultStatus_AuthenticationNotAllowed = 8,
                    DevicePairingResultStatus_AuthenticationFailure = 9,
                    DevicePairingResultStatus_NoSupportedProfiles = 10,
                    DevicePairingResultStatus_ProtectionLevelCouldNotBeMet = 11,
                    DevicePairingResultStatus_AccessDenied = 12,
                    DevicePairingResultStatus_InvalidCeremonyData = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_PairingCanceled = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_OperationAlreadyInProgress = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_RequiredHandlerNotRegistered = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_RejectedByHandler = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_RemoteDeviceHasAssociation = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    DevicePairingResultStatus_Failed = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DevicePickerDisplayStatusOptions : unsigned int
                {
                    DevicePickerDisplayStatusOptions_None = 0,
                    DevicePickerDisplayStatusOptions_ShowProgress = 0x1,
                    DevicePickerDisplayStatusOptions_ShowDisconnectButton = 0x2,
                    DevicePickerDisplayStatusOptions_ShowRetryButton = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(DevicePickerDisplayStatusOptions)
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceUnpairingResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceUnpairingResultStatus : int
                {
                    DeviceUnpairingResultStatus_Unpaired = 0,
                    DeviceUnpairingResultStatus_AlreadyUnpaired = 1,
                    DeviceUnpairingResultStatus_OperationAlreadyInProgress = 2,
                    DeviceUnpairingResultStatus_AccessDenied = 3,
                    DeviceUnpairingResultStatus_Failed = 4,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceWatcherEventKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceWatcherEventKind : int
                {
                    DeviceWatcherEventKind_Add = 0,
                    DeviceWatcherEventKind_Update = 1,
                    DeviceWatcherEventKind_Remove = 2,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum DeviceWatcherStatus : int
                {
                    DeviceWatcherStatus_Created = 0,
                    DeviceWatcherStatus_Started = 1,
                    DeviceWatcherStatus_EnumerationCompleted = 2,
                    DeviceWatcherStatus_Stopping = 3,
                    DeviceWatcherStatus_Stopped = 4,
                    DeviceWatcherStatus_Aborted = 5,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.Panel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                enum Panel : int
                {
                    Panel_Unknown = 0,
                    Panel_Front = 1,
                    Panel_Back = 2,
                    Panel_Top = 3,
                    Panel_Bottom = 4,
                    Panel_Left = 5,
                    Panel_Right = 6,
                };
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("deda0bcc-4f9d-4f58-9dba-a9bc800408d5")
                IDeviceAccessChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Enumeration::DeviceAccessStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessChangedEventArgs = __uuidof(IDeviceAccessChangedEventArgs);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs2[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("82523262-934b-4b30-a178-adc39f2f2be3")
                IDeviceAccessChangedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessChangedEventArgs2 = __uuidof(IDeviceAccessChangedEventArgs2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs3[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("7580a878-7fd9-5cd7-8560-3c644b9b10db")
                IDeviceAccessChangedEventArgs3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserPromptRequired(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessChangedEventArgs3 = __uuidof(IDeviceAccessChangedEventArgs3);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformation[] = L"Windows.Devices.Enumeration.IDeviceAccessInformation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("0baa9a73-6de5-4915-8ddd-9a0554a6f545")
                IDeviceAccessInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AccessChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AccessChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentStatus(
                        ABI::Windows::Devices::Enumeration::DeviceAccessStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessInformation = __uuidof(IDeviceAccessInformation);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformation2[] = L"Windows.Devices.Enumeration.IDeviceAccessInformation2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("e2b9dff6-e88f-5d0a-9c1e-d788808df47b")
                IDeviceAccessInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserPromptRequired(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessInformation2 = __uuidof(IDeviceAccessInformation2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformationStatics[] = L"Windows.Devices.Enumeration.IDeviceAccessInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("574bd3d3-5f30-45cd-8a94-724fe5973084")
                IDeviceAccessInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromId(
                        HSTRING deviceId,
                        ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromDeviceClassId(
                        GUID deviceClassId,
                        ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromDeviceClass(
                        ABI::Windows::Devices::Enumeration::DeviceClass deviceClass,
                        ABI::Windows::Devices::Enumeration::IDeviceAccessInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceAccessInformationStatics = __uuidof(IDeviceAccessInformationStatics);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceConnectionChangeTriggerDetails[] = L"Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("b8578c0c-bbc1-484b-bffa-7b31dcc200b2")
                IDeviceConnectionChangeTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceConnectionChangeTriggerDetails = __uuidof(IDeviceConnectionChangeTriggerDetails);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceDisconnectButtonClickedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("8e44b56d-f902-4a00-b536-f37992e6a2a7")
                IDeviceDisconnectButtonClickedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceDisconnectButtonClickedEventArgs = __uuidof(IDeviceDisconnectButtonClickedEventArgs);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceEnumerationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceEnumerationSettings[] = L"Windows.Devices.Enumeration.IDeviceEnumerationSettings";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("f7710f66-9ff3-41c8-85eb-87f81148a30f")
                IDeviceEnumerationSettings : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IDeviceEnumerationSettings = __uuidof(IDeviceEnumerationSettings);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformation[] = L"Windows.Devices.Enumeration.IDeviceInformation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("aba0fb95-4398-489d-8e44-e6130927011f")
                IDeviceInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDefault(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EnclosureLocation(
                        ABI::Windows::Devices::Enumeration::IEnclosureLocation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Update(
                        ABI::Windows::Devices::Enumeration::IDeviceInformationUpdate* updateInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGlyphThumbnailAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail** asyncOp
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformation = __uuidof(IDeviceInformation);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformation2[] = L"Windows.Devices.Enumeration.IDeviceInformation2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("f156a638-7997-48d9-a10c-269d46533f48")
                IDeviceInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pairing(
                        ABI::Windows::Devices::Enumeration::IDeviceInformationPairing** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformation2 = __uuidof(IDeviceInformation2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationCustomPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationCustomPairing[] = L"Windows.Devices.Enumeration.IDeviceInformationCustomPairing";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("85138c02-4ee6-4914-8370-107a39144c0e")
                IDeviceInformationCustomPairing : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE PairAsync(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds pairingKindsSupported,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PairWithProtectionLevelAsync(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds pairingKindsSupported,
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel minProtectionLevel,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PairWithProtectionLevelAndSettingsAsync(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds pairingKindsSupported,
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel minProtectionLevel,
                        ABI::Windows::Devices::Enumeration::IDevicePairingSettings* devicePairingSettings,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PairingRequested(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PairingRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationCustomPairing = __uuidof(IDeviceInformationCustomPairing);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationCustomPairing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationCustomPairing2[] = L"Windows.Devices.Enumeration.IDeviceInformationCustomPairing2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("0ebda662-e696-5fa9-8f72-80cfebcd851d")
                IDeviceInformationCustomPairing2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddPairingSetMember(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PairingSetMembersRequested(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PairingSetMembersRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationCustomPairing2 = __uuidof(IDeviceInformationCustomPairing2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairing[] = L"Windows.Devices.Enumeration.IDeviceInformationPairing";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("2c4769f5-f684-40d5-8469-e8dbaab70485")
                IDeviceInformationPairing : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsPaired(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanPair(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PairAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PairWithProtectionLevelAsync(
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel minProtectionLevel,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationPairing = __uuidof(IDeviceInformationPairing);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairing2[] = L"Windows.Devices.Enumeration.IDeviceInformationPairing2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("f68612fd-0aee-4328-85cc-1c742bb1790d")
                IDeviceInformationPairing2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProtectionLevel(
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Custom(
                        ABI::Windows::Devices::Enumeration::IDeviceInformationCustomPairing** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PairWithProtectionLevelAndSettingsAsync(
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel minProtectionLevel,
                        ABI::Windows::Devices::Enumeration::IDevicePairingSettings* devicePairingSettings,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnpairAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationPairing2 = __uuidof(IDeviceInformationPairing2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairingStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairingStatics[] = L"Windows.Devices.Enumeration.IDeviceInformationPairingStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("e915c408-36d4-49a1-bf13-514173799b6b")
                IDeviceInformationPairingStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryRegisterForAllInboundPairingRequests(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds pairingKindsSupported,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationPairingStatics = __uuidof(IDeviceInformationPairingStatics);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairingStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairingStatics2[] = L"Windows.Devices.Enumeration.IDeviceInformationPairingStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("04de5372-b7b7-476b-a74f-c5836a704d98")
                IDeviceInformationPairingStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryRegisterForAllInboundPairingRequestsWithProtectionLevel(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds pairingKindsSupported,
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel minProtectionLevel,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationPairingStatics2 = __uuidof(IDeviceInformationPairingStatics2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("c17f100e-3a46-4a78-8013-769dc9b97390")
                IDeviceInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdAsyncAdditionalProperties(
                        HSTRING deviceId,
                        __FIIterable_1_HSTRING* additionalProperties,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsyncDeviceClass(
                        ABI::Windows::Devices::Enumeration::DeviceClass deviceClass,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsyncAqsFilter(
                        HSTRING aqsFilter,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsyncAqsFilterAndAdditionalProperties(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** watcher
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherDeviceClass(
                        ABI::Windows::Devices::Enumeration::DeviceClass deviceClass,
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** watcher
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherAqsFilter(
                        HSTRING aqsFilter,
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** watcher
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherAqsFilterAndAdditionalProperties(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** watcher
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationStatics = __uuidof(IDeviceInformationStatics);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics2[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("493b4f34-a84f-45fd-9167-15d1cb1bd1f9")
                IDeviceInformationStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAqsFilterFromDeviceClass(
                        ABI::Windows::Devices::Enumeration::DeviceClass deviceClass,
                        HSTRING* aqsFilter
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdAsyncWithKindAndAdditionalProperties(
                        HSTRING deviceId,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherWithKindAqsFilterAndAdditionalProperties(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** watcher
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationStatics2 = __uuidof(IDeviceInformationStatics2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics3[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("25f06279-9364-5a6c-8a54-5d4a6d3d922a")
                IDeviceInformationStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings(
                        HSTRING deviceId,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        ABI::Windows::Devices::Enumeration::IDeviceEnumerationSettings* settings,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        ABI::Windows::Devices::Enumeration::IDeviceEnumerationSettings* settings,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings(
                        HSTRING aqsFilter,
                        __FIIterable_1_HSTRING* additionalProperties,
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind kind,
                        ABI::Windows::Devices::Enumeration::IDeviceEnumerationSettings* settings,
                        ABI::Windows::Devices::Enumeration::IDeviceWatcher** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationStatics3 = __uuidof(IDeviceInformationStatics3);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationUpdate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationUpdate[] = L"Windows.Devices.Enumeration.IDeviceInformationUpdate";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("8f315305-d972-44b7-a37e-9e822c78213b")
                IDeviceInformationUpdate : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationUpdate = __uuidof(IDeviceInformationUpdate);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationUpdate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationUpdate2[] = L"Windows.Devices.Enumeration.IDeviceInformationUpdate2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("5d9d148c-a873-485e-baa6-aa620788e3cc")
                IDeviceInformationUpdate2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Devices::Enumeration::DeviceInformationKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceInformationUpdate2 = __uuidof(IDeviceInformationUpdate2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("f717fc56-de6b-487f-8376-0180aca69963")
                IDevicePairingRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PairingKind(
                        ABI::Windows::Devices::Enumeration::DevicePairingKinds* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pin(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Accept(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptWithPin(
                        HSTRING pin
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingRequestedEventArgs = __uuidof(IDevicePairingRequestedEventArgs);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs2[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("c83752d9-e4d3-4db0-a360-a105e437dbdc")
                IDevicePairingRequestedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AcceptWithPasswordCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingRequestedEventArgs2 = __uuidof(IDevicePairingRequestedEventArgs2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs3[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("195e5a38-43dc-562f-babe-efc8b110088b")
                IDevicePairingRequestedEventArgs3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AcceptWithAddress(
                        HSTRING address
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingRequestedEventArgs3 = __uuidof(IDevicePairingRequestedEventArgs3);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingResult[] = L"Windows.Devices.Enumeration.IDevicePairingResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("072b02bf-dd95-4025-9b37-de51adba37b7")
                IDevicePairingResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Enumeration::DevicePairingResultStatus* status
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtectionLevelUsed(
                        ABI::Windows::Devices::Enumeration::DevicePairingProtectionLevel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingResult = __uuidof(IDevicePairingResult);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingSetMembersRequestedEventArgs[] = L"Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("7fb42cff-ecac-5012-8d7d-a1894680a349")
                IDevicePairingSetMembersRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Enumeration::DevicePairingAddPairingSetMemberStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ParentDeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PairingSetMembers(
                        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingSetMembersRequestedEventArgs = __uuidof(IDevicePairingSetMembersRequestedEventArgs);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingSettings[] = L"Windows.Devices.Enumeration.IDevicePairingSettings";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("482cb27c-83bb-420e-be51-6602b222de54")
                IDevicePairingSettings : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IDevicePairingSettings = __uuidof(IDevicePairingSettings);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePicker[] = L"Windows.Devices.Enumeration.IDevicePicker";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("84997aa2-034a-4440-8813-7d0bd479bf5a")
                IDevicePicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Filter(
                        ABI::Windows::Devices::Enumeration::IDevicePickerFilter** filter
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Appearance(
                        ABI::Windows::Devices::Enumeration::IDevicePickerAppearance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedProperties(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DeviceSelected(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DeviceSelected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DisconnectButtonClicked(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DisconnectButtonClicked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DevicePickerDismissed(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DevicePickerDismissed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Show(
                        ABI::Windows::Foundation::Rect selection
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowWithPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement placement
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleDeviceAsync(
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleDeviceAsyncWithPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement placement,
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Hide(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDisplayStatus(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device,
                        HSTRING status,
                        ABI::Windows::Devices::Enumeration::DevicePickerDisplayStatusOptions options
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePicker = __uuidof(IDevicePicker);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePickerAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePickerAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePickerAppearance[] = L"Windows.Devices.Enumeration.IDevicePickerAppearance";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("e69a12c6-e627-4ed8-9b6c-460af445e56d")
                IDevicePickerAppearance : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AccentColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AccentColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedForegroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SelectedForegroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedBackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SelectedBackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedAccentColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SelectedAccentColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePickerAppearance = __uuidof(IDevicePickerAppearance);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePickerFilter[] = L"Windows.Devices.Enumeration.IDevicePickerFilter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("91db92a2-57cb-48f1-9b59-a59b7a1f02a2")
                IDevicePickerFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedDeviceClasses(
                        __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedDeviceSelectors(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePickerFilter = __uuidof(IDevicePickerFilter);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceSelectedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceSelectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("269edade-1d2f-4940-8402-4156b81d3c77")
                IDeviceSelectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedDevice(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceSelectedEventArgs = __uuidof(IDeviceSelectedEventArgs);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceUnpairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceUnpairingResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceUnpairingResult[] = L"Windows.Devices.Enumeration.IDeviceUnpairingResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("66f44ad3-79d9-444b-92cf-a92ef72571c7")
                IDeviceUnpairingResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Enumeration::DeviceUnpairingResultStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceUnpairingResult = __uuidof(IDeviceUnpairingResult);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcher[] = L"Windows.Devices.Enumeration.IDeviceWatcher";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("c9eab97d-8f6b-4f96-a9f4-abc814e22271")
                IDeviceWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Updated(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Devices::Enumeration::DeviceWatcherStatus* status
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceWatcher = __uuidof(IDeviceWatcher);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcher2[] = L"Windows.Devices.Enumeration.IDeviceWatcher2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("ff08456e-ed14-49e9-9a69-8117c54ae971")
                IDeviceWatcher2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetBackgroundTrigger(
                        __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* requestedEventKinds,
                        ABI::Windows::ApplicationModel::Background::IDeviceWatcherTrigger** trigger
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceWatcher2 = __uuidof(IDeviceWatcher2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcherEvent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcherEvent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcherEvent[] = L"Windows.Devices.Enumeration.IDeviceWatcherEvent";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("74aa9c0b-1dbd-47fd-b635-3cc556d0ff8b")
                IDeviceWatcherEvent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Devices::Enumeration::DeviceWatcherEventKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformationUpdate(
                        ABI::Windows::Devices::Enumeration::IDeviceInformationUpdate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceWatcherEvent = __uuidof(IDeviceWatcherEvent);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcherTriggerDetails[] = L"Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("38808119-4cb7-4e57-a56d-776d07cbfef9")
                IDeviceWatcherTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceWatcherEvents(
                        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceWatcherTriggerDetails = __uuidof(IDeviceWatcherTriggerDetails);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IEnclosureLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.EnclosureLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IEnclosureLocation[] = L"Windows.Devices.Enumeration.IEnclosureLocation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("42340a27-5810-459c-aabb-c65e1f813ecf")
                IEnclosureLocation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InDock(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InLid(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Panel(
                        ABI::Windows::Devices::Enumeration::Panel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEnclosureLocation = __uuidof(IEnclosureLocation);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IEnclosureLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.EnclosureLocation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Enumeration.IEnclosureLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IEnclosureLocation2[] = L"Windows.Devices.Enumeration.IEnclosureLocation2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                MIDL_INTERFACE("2885995b-e07d-485d-8a9e-bdf29aef4f66")
                IEnclosureLocation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RotationAngleInDegreesClockwise(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEnclosureLocation2 = __uuidof(IEnclosureLocation2);
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs[] = L"Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceAccessInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceAccessInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceAccessInformation ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceAccessInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceAccessInformation[] = L"Windows.Devices.Enumeration.DeviceAccessInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails[] = L"Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs[] = L"Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics3 interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformation ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformation[] = L"Windows.Devices.Enumeration.DeviceInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Enumeration.DeviceInformation> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceInformation>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCollection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationCollection[] = L"Windows.Devices.Enumeration.DeviceInformationCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationCustomPairing ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationCustomPairing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCustomPairing_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCustomPairing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationCustomPairing[] = L"Windows.Devices.Enumeration.DeviceInformationCustomPairing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationPairingStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationPairingStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationPairing ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationPairing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationPairing_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationPairing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationPairing[] = L"Windows.Devices.Enumeration.DeviceInformationPairing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationUpdate ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationUpdate2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationUpdate_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationUpdate[] = L"Windows.Devices.Enumeration.DeviceInformationUpdate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs ** Default Interface **
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs[] = L"Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingResult[] = L"Windows.Devices.Enumeration.DevicePairingResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs[] = L"Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePicker[] = L"Windows.Devices.Enumeration.DevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePickerAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePickerAppearance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerAppearance_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerAppearance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePickerAppearance[] = L"Windows.Devices.Enumeration.DevicePickerAppearance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePickerFilter[] = L"Windows.Devices.Enumeration.DevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceSelectedEventArgs[] = L"Windows.Devices.Enumeration.DeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceThumbnail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceThumbnail_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceThumbnail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceThumbnail[] = L"Windows.Devices.Enumeration.DeviceThumbnail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceUnpairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceUnpairingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceUnpairingResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceUnpairingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceUnpairingResult[] = L"Windows.Devices.Enumeration.DeviceUnpairingResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcher ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceWatcher2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcher[] = L"Windows.Devices.Enumeration.DeviceWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcherEvent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcherEvent ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherEvent_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherEvent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcherEvent[] = L"Windows.Devices.Enumeration.DeviceWatcherEvent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails[] = L"Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.EnclosureLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IEnclosureLocation ** Default Interface **
 *    Windows.Devices.Enumeration.IEnclosureLocation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_EnclosureLocation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_EnclosureLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_EnclosureLocation[] = L"Windows.Devices.Enumeration.EnclosureLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2 __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3 __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2 __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2 __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDevicePairingResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass;

typedef struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl;

interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass;

typedef struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceClass** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl;

interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

typedef struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl;

interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

typedef struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl;

interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind;

typedef struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl;

interface __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind;

typedef struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* This,
        __FIIterator_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl;

interface __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass;

typedef struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl;

interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent;

typedef struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl;

interface __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass;

typedef struct __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceClass** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass* items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl;

interface __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* sender,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdateVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger __x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingAddPairingSetMemberStatus __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingAddPairingSetMemberStatus;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingResultStatus __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingResultStatus;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePickerDisplayStatusOptions __x_ABI_CWindows_CDevices_CEnumeration_CDevicePickerDisplayStatusOptions;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceUnpairingResultStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceUnpairingResultStatus;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherStatus;

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel __x_ABI_CWindows_CDevices_CEnumeration_CPanel;

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus
{
    DeviceAccessStatus_Unspecified = 0,
    DeviceAccessStatus_Allowed = 1,
    DeviceAccessStatus_DeniedByUser = 2,
    DeviceAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass
{
    DeviceClass_All = 0,
    DeviceClass_AudioCapture = 1,
    DeviceClass_AudioRender = 2,
    DeviceClass_PortableStorageDevice = 3,
    DeviceClass_VideoCapture = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DeviceClass_ImageScanner = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DeviceClass_Location = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceInformationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind
{
    DeviceInformationKind_Unknown = 0,
    DeviceInformationKind_DeviceInterface = 1,
    DeviceInformationKind_DeviceContainer = 2,
    DeviceInformationKind_Device = 3,
    DeviceInformationKind_DeviceInterfaceClass = 4,
    DeviceInformationKind_AssociationEndpoint = 5,
    DeviceInformationKind_AssociationEndpointContainer = 6,
    DeviceInformationKind_AssociationEndpointService = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DeviceInformationKind_DevicePanel = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    DeviceInformationKind_AssociationEndpointProtocol = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingAddPairingSetMemberStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingAddPairingSetMemberStatus
{
    DevicePairingAddPairingSetMemberStatus_AddedToSet = 0,
    DevicePairingAddPairingSetMemberStatus_CouldNotBeAddedToSet = 1,
    DevicePairingAddPairingSetMemberStatus_SetDiscoveryNotAttemptedByProtocol = 2,
    DevicePairingAddPairingSetMemberStatus_SetDiscoveryCompletedByProtocol = 3,
    DevicePairingAddPairingSetMemberStatus_SetDiscoveryPartiallyCompletedByProtocol = 4,
    DevicePairingAddPairingSetMemberStatus_Failed = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds
{
    DevicePairingKinds_None = 0,
    DevicePairingKinds_ConfirmOnly = 0x1,
    DevicePairingKinds_DisplayPin = 0x2,
    DevicePairingKinds_ProvidePin = 0x4,
    DevicePairingKinds_ConfirmPinMatch = 0x8,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
    DevicePairingKinds_ProvidePasswordCredential = 0x10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    DevicePairingKinds_ProvideAddress = 0x20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel
{
    DevicePairingProtectionLevel_Default = 0,
    DevicePairingProtectionLevel_None = 1,
    DevicePairingProtectionLevel_Encryption = 2,
    DevicePairingProtectionLevel_EncryptionAndAuthentication = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePairingResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingResultStatus
{
    DevicePairingResultStatus_Paired = 0,
    DevicePairingResultStatus_NotReadyToPair = 1,
    DevicePairingResultStatus_NotPaired = 2,
    DevicePairingResultStatus_AlreadyPaired = 3,
    DevicePairingResultStatus_ConnectionRejected = 4,
    DevicePairingResultStatus_TooManyConnections = 5,
    DevicePairingResultStatus_HardwareFailure = 6,
    DevicePairingResultStatus_AuthenticationTimeout = 7,
    DevicePairingResultStatus_AuthenticationNotAllowed = 8,
    DevicePairingResultStatus_AuthenticationFailure = 9,
    DevicePairingResultStatus_NoSupportedProfiles = 10,
    DevicePairingResultStatus_ProtectionLevelCouldNotBeMet = 11,
    DevicePairingResultStatus_AccessDenied = 12,
    DevicePairingResultStatus_InvalidCeremonyData = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_PairingCanceled = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_OperationAlreadyInProgress = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_RequiredHandlerNotRegistered = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_RejectedByHandler = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_RemoteDeviceHasAssociation = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    DevicePairingResultStatus_Failed = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePickerDisplayStatusOptions
{
    DevicePickerDisplayStatusOptions_None = 0,
    DevicePickerDisplayStatusOptions_ShowProgress = 0x1,
    DevicePickerDisplayStatusOptions_ShowDisconnectButton = 0x2,
    DevicePickerDisplayStatusOptions_ShowRetryButton = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceUnpairingResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceUnpairingResultStatus
{
    DeviceUnpairingResultStatus_Unpaired = 0,
    DeviceUnpairingResultStatus_AlreadyUnpaired = 1,
    DeviceUnpairingResultStatus_OperationAlreadyInProgress = 2,
    DeviceUnpairingResultStatus_AccessDenied = 3,
    DeviceUnpairingResultStatus_Failed = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceWatcherEventKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind
{
    DeviceWatcherEventKind_Add = 0,
    DeviceWatcherEventKind_Update = 1,
    DeviceWatcherEventKind_Remove = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.DeviceWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherStatus
{
    DeviceWatcherStatus_Created = 0,
    DeviceWatcherStatus_Started = 1,
    DeviceWatcherStatus_EnumerationCompleted = 2,
    DeviceWatcherStatus_Stopping = 3,
    DeviceWatcherStatus_Stopped = 4,
    DeviceWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Enumeration.Panel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel
{
    Panel_Unknown = 0,
    Panel_Front = 1,
    Panel_Back = 2,
    Panel_Top = 3,
    Panel_Bottom = 4,
    Panel_Left = 5,
    Panel_Right = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs2[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessChangedEventArgs3[] = L"Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserPromptRequired)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_get_UserPromptRequired(This, value) \
    ((This)->lpVtbl->get_UserPromptRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessChangedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformation[] = L"Windows.Devices.Enumeration.IDeviceAccessInformation";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AccessChanged)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceAccessInformation_Windows__CDevices__CEnumeration__CDeviceAccessChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_AccessChanged)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_CurrentStatus)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_add_AccessChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_AccessChanged(This, handler, cookie))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_remove_AccessChanged(This, cookie) \
    ((This)->lpVtbl->remove_AccessChanged(This, cookie))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_get_CurrentStatus(This, status) \
    ((This)->lpVtbl->get_CurrentStatus(This, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformation2[] = L"Windows.Devices.Enumeration.IDeviceAccessInformation2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserPromptRequired)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_get_UserPromptRequired(This, value) \
    ((This)->lpVtbl->get_UserPromptRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceAccessInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceAccessInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceAccessInformationStatics[] = L"Windows.Devices.Enumeration.IDeviceAccessInformationStatics";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromId)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromDeviceClassId)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        GUID deviceClassId,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromDeviceClass)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass deviceClass,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_CreateFromId(This, deviceId, value) \
    ((This)->lpVtbl->CreateFromId(This, deviceId, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_CreateFromDeviceClassId(This, deviceClassId, value) \
    ((This)->lpVtbl->CreateFromDeviceClassId(This, deviceClassId, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_CreateFromDeviceClass(This, deviceClass, value) \
    ((This)->lpVtbl->CreateFromDeviceClass(This, deviceClass, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceAccessInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceConnectionChangeTriggerDetails[] = L"Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceConnectionChangeTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceDisconnectButtonClickedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceEnumerationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceEnumerationSettings[] = L"Windows.Devices.Enumeration.IDeviceEnumerationSettings";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettingsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformation[] = L"Windows.Devices.Enumeration.IDeviceInformation";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDefault)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_EnclosureLocation)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        __FIMapView_2_HSTRING_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* updateInfo);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail** asyncOp);
    HRESULT (STDMETHODCALLTYPE* GetGlyphThumbnailAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceThumbnail** asyncOp);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_IsDefault(This, value) \
    ((This)->lpVtbl->get_IsDefault(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_EnclosureLocation(This, value) \
    ((This)->lpVtbl->get_EnclosureLocation(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_Update(This, updateInfo) \
    ((This)->lpVtbl->Update(This, updateInfo))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_GetThumbnailAsync(This, asyncOp) \
    ((This)->lpVtbl->GetThumbnailAsync(This, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_GetGlyphThumbnailAsync(This, asyncOp) \
    ((This)->lpVtbl->GetGlyphThumbnailAsync(This, asyncOp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformation2[] = L"Windows.Devices.Enumeration.IDeviceInformation2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Pairing)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_get_Pairing(This, value) \
    ((This)->lpVtbl->get_Pairing(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationCustomPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationCustomPairing[] = L"Windows.Devices.Enumeration.IDeviceInformationCustomPairing";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PairAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds pairingKindsSupported,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* PairWithProtectionLevelAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds pairingKindsSupported,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel minProtectionLevel,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* PairWithProtectionLevelAndSettingsAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds pairingKindsSupported,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel minProtectionLevel,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* devicePairingSettings,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* add_PairingRequested)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PairingRequested)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairingVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_PairAsync(This, pairingKindsSupported, result) \
    ((This)->lpVtbl->PairAsync(This, pairingKindsSupported, result))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_PairWithProtectionLevelAsync(This, pairingKindsSupported, minProtectionLevel, result) \
    ((This)->lpVtbl->PairWithProtectionLevelAsync(This, pairingKindsSupported, minProtectionLevel, result))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_PairWithProtectionLevelAndSettingsAsync(This, pairingKindsSupported, minProtectionLevel, devicePairingSettings, result) \
    ((This)->lpVtbl->PairWithProtectionLevelAndSettingsAsync(This, pairingKindsSupported, minProtectionLevel, devicePairingSettings, result))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_add_PairingRequested(This, handler, token) \
    ((This)->lpVtbl->add_PairingRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_remove_PairingRequested(This, token) \
    ((This)->lpVtbl->remove_PairingRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationCustomPairing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationCustomPairing2[] = L"Windows.Devices.Enumeration.IDeviceInformationCustomPairing2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddPairingSetMember)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device);
    HRESULT (STDMETHODCALLTYPE* add_PairingSetMembersRequested)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceInformationCustomPairing_Windows__CDevices__CEnumeration__CDevicePairingSetMembersRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PairingSetMembersRequested)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_AddPairingSetMember(This, device) \
    ((This)->lpVtbl->AddPairingSetMember(This, device))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_add_PairingSetMembersRequested(This, handler, token) \
    ((This)->lpVtbl->add_PairingSetMembersRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_remove_PairingSetMembersRequested(This, token) \
    ((This)->lpVtbl->remove_PairingSetMembersRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairing[] = L"Windows.Devices.Enumeration.IDeviceInformationPairing";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPaired)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanPair)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* PairAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* PairWithProtectionLevelAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel minProtectionLevel,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_get_IsPaired(This, value) \
    ((This)->lpVtbl->get_IsPaired(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_get_CanPair(This, value) \
    ((This)->lpVtbl->get_CanPair(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_PairAsync(This, result) \
    ((This)->lpVtbl->PairAsync(This, result))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_PairWithProtectionLevelAsync(This, minProtectionLevel, result) \
    ((This)->lpVtbl->PairWithProtectionLevelAsync(This, minProtectionLevel, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairing2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairing2[] = L"Windows.Devices.Enumeration.IDeviceInformationPairing2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProtectionLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* get_Custom)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationCustomPairing** value);
    HRESULT (STDMETHODCALLTYPE* PairWithProtectionLevelAndSettingsAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel minProtectionLevel,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* devicePairingSettings,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDevicePairingResult** result);
    HRESULT (STDMETHODCALLTYPE* UnpairAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceUnpairingResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_get_ProtectionLevel(This, value) \
    ((This)->lpVtbl->get_ProtectionLevel(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_get_Custom(This, value) \
    ((This)->lpVtbl->get_Custom(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_PairWithProtectionLevelAndSettingsAsync(This, minProtectionLevel, devicePairingSettings, result) \
    ((This)->lpVtbl->PairWithProtectionLevelAndSettingsAsync(This, minProtectionLevel, devicePairingSettings, result))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_UnpairAsync(This, result) \
    ((This)->lpVtbl->UnpairAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairing2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairingStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairingStatics[] = L"Windows.Devices.Enumeration.IDeviceInformationPairingStatics";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryRegisterForAllInboundPairingRequests)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds pairingKindsSupported,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_TryRegisterForAllInboundPairingRequests(This, pairingKindsSupported, result) \
    ((This)->lpVtbl->TryRegisterForAllInboundPairingRequests(This, pairingKindsSupported, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationPairingStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationPairing
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationPairingStatics2[] = L"Windows.Devices.Enumeration.IDeviceInformationPairingStatics2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryRegisterForAllInboundPairingRequestsWithProtectionLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds pairingKindsSupported,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel minProtectionLevel,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_TryRegisterForAllInboundPairingRequestsWithProtectionLevel(This, pairingKindsSupported, minProtectionLevel, result) \
    ((This)->lpVtbl->TryRegisterForAllInboundPairingRequestsWithProtectionLevel(This, pairingKindsSupported, minProtectionLevel, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationPairingStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdAsyncAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING deviceId,
        __FIIterable_1_HSTRING* additionalProperties,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncDeviceClass)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass deviceClass,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncAqsFilter)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING aqsFilter,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncAqsFilterAndAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** watcher);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherDeviceClass)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass deviceClass,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** watcher);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherAqsFilter)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING aqsFilter,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** watcher);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherAqsFilterAndAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** watcher);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateFromIdAsync(This, deviceId, asyncOp) \
    ((This)->lpVtbl->CreateFromIdAsync(This, deviceId, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateFromIdAsyncAdditionalProperties(This, deviceId, additionalProperties, asyncOp) \
    ((This)->lpVtbl->CreateFromIdAsyncAdditionalProperties(This, deviceId, additionalProperties, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FindAllAsync(This, asyncOp) \
    ((This)->lpVtbl->FindAllAsync(This, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FindAllAsyncDeviceClass(This, deviceClass, asyncOp) \
    ((This)->lpVtbl->FindAllAsyncDeviceClass(This, deviceClass, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FindAllAsyncAqsFilter(This, aqsFilter, asyncOp) \
    ((This)->lpVtbl->FindAllAsyncAqsFilter(This, aqsFilter, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_FindAllAsyncAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, asyncOp) \
    ((This)->lpVtbl->FindAllAsyncAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateWatcher(This, watcher) \
    ((This)->lpVtbl->CreateWatcher(This, watcher))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateWatcherDeviceClass(This, deviceClass, watcher) \
    ((This)->lpVtbl->CreateWatcherDeviceClass(This, deviceClass, watcher))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateWatcherAqsFilter(This, aqsFilter, watcher) \
    ((This)->lpVtbl->CreateWatcherAqsFilter(This, aqsFilter, watcher))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_CreateWatcherAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, watcher) \
    ((This)->lpVtbl->CreateWatcherAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, watcher))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics2[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAqsFilterFromDeviceClass)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceClass deviceClass,
        HSTRING* aqsFilter);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdAsyncWithKindAndAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        HSTRING deviceId,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** asyncOp);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncWithKindAqsFilterAndAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** asyncOp);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherWithKindAqsFilterAndAdditionalProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** watcher);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_GetAqsFilterFromDeviceClass(This, deviceClass, aqsFilter) \
    ((This)->lpVtbl->GetAqsFilterFromDeviceClass(This, deviceClass, aqsFilter))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_CreateFromIdAsyncWithKindAndAdditionalProperties(This, deviceId, additionalProperties, kind, asyncOp) \
    ((This)->lpVtbl->CreateFromIdAsyncWithKindAndAdditionalProperties(This, deviceId, additionalProperties, kind, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_FindAllAsyncWithKindAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, kind, asyncOp) \
    ((This)->lpVtbl->FindAllAsyncWithKindAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, kind, asyncOp))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_CreateWatcherWithKindAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, kind, watcher) \
    ((This)->lpVtbl->CreateWatcherWithKindAqsFilterAndAdditionalProperties(This, aqsFilter, additionalProperties, kind, watcher))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationStatics3[] = L"Windows.Devices.Enumeration.IDeviceInformationStatics3";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        HSTRING deviceId,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* settings,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* settings,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformationCollection** operation);
    HRESULT (STDMETHODCALLTYPE* CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3* This,
        HSTRING aqsFilter,
        __FIIterable_1_HSTRING* additionalProperties,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind kind,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceEnumerationSettings* settings,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings(This, deviceId, additionalProperties, kind, settings, operation) \
    ((This)->lpVtbl->CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings(This, deviceId, additionalProperties, kind, settings, operation))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings(This, aqsFilter, additionalProperties, kind, settings, operation) \
    ((This)->lpVtbl->FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings(This, aqsFilter, additionalProperties, kind, settings, operation))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings(This, aqsFilter, additionalProperties, kind, settings, result) \
    ((This)->lpVtbl->CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings(This, aqsFilter, additionalProperties, kind, settings, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationUpdate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationUpdate[] = L"Windows.Devices.Enumeration.IDeviceInformationUpdate";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdateVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceInformationUpdate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceInformationUpdate2[] = L"Windows.Devices.Enumeration.IDeviceInformationUpdate2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceInformationKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_PairingKind)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingKinds* value);
    HRESULT (STDMETHODCALLTYPE* get_Pin)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* Accept)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* AcceptWithPin)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        HSTRING pin);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_get_PairingKind(This, value) \
    ((This)->lpVtbl->get_PairingKind(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_get_Pin(This, value) \
    ((This)->lpVtbl->get_Pin(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_Accept(This) \
    ((This)->lpVtbl->Accept(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_AcceptWithPin(This, pin) \
    ((This)->lpVtbl->AcceptWithPin(This, pin))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs2[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AcceptWithPasswordCredential)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_AcceptWithPasswordCredential(This, passwordCredential) \
    ((This)->lpVtbl->AcceptWithPasswordCredential(This, passwordCredential))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingRequestedEventArgs3[] = L"Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AcceptWithAddress)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3* This,
        HSTRING address);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_AcceptWithAddress(This, address) \
    ((This)->lpVtbl->AcceptWithAddress(This, address))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingRequestedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingResult[] = L"Windows.Devices.Enumeration.IDevicePairingResult";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingResultStatus* status);
    HRESULT (STDMETHODCALLTYPE* get_ProtectionLevelUsed)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingProtectionLevel* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResultVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_get_ProtectionLevelUsed(This, value) \
    ((This)->lpVtbl->get_ProtectionLevelUsed(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingSetMembersRequestedEventArgs[] = L"Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePairingAddPairingSetMemberStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentDeviceInformation)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_PairingSetMembers)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs* This,
        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_get_ParentDeviceInformation(This, value) \
    ((This)->lpVtbl->get_ParentDeviceInformation(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_get_PairingSetMembers(This, value) \
    ((This)->lpVtbl->get_PairingSetMembers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSetMembersRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePairingSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePairingSettings[] = L"Windows.Devices.Enumeration.IDevicePairingSettings";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettingsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePairingSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePicker[] = L"Windows.Devices.Enumeration.IDevicePicker";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Filter)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter** filter);
    HRESULT (STDMETHODCALLTYPE* get_Appearance)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedProperties)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* add_DeviceSelected)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceSelectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DeviceSelected)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DisconnectButtonClicked)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_Windows__CDevices__CEnumeration__CDeviceDisconnectButtonClickedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisconnectButtonClicked)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DevicePickerDismissed)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDevicePicker_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DevicePickerDismissed)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection);
    HRESULT (STDMETHODCALLTYPE* ShowWithPlacement)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement placement);
    HRESULT (STDMETHODCALLTYPE* PickSingleDeviceAsync)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation);
    HRESULT (STDMETHODCALLTYPE* PickSingleDeviceAsyncWithPlacement)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement placement,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceInformation** operation);
    HRESULT (STDMETHODCALLTYPE* Hide)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This);
    HRESULT (STDMETHODCALLTYPE* SetDisplayStatus)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device,
        HSTRING status,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDevicePickerDisplayStatusOptions options);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_get_Filter(This, filter) \
    ((This)->lpVtbl->get_Filter(This, filter))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_get_Appearance(This, value) \
    ((This)->lpVtbl->get_Appearance(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_get_RequestedProperties(This, value) \
    ((This)->lpVtbl->get_RequestedProperties(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_add_DeviceSelected(This, handler, token) \
    ((This)->lpVtbl->add_DeviceSelected(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_remove_DeviceSelected(This, token) \
    ((This)->lpVtbl->remove_DeviceSelected(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_add_DisconnectButtonClicked(This, handler, token) \
    ((This)->lpVtbl->add_DisconnectButtonClicked(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_remove_DisconnectButtonClicked(This, token) \
    ((This)->lpVtbl->remove_DisconnectButtonClicked(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_add_DevicePickerDismissed(This, handler, token) \
    ((This)->lpVtbl->add_DevicePickerDismissed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_remove_DevicePickerDismissed(This, token) \
    ((This)->lpVtbl->remove_DevicePickerDismissed(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_Show(This, selection) \
    ((This)->lpVtbl->Show(This, selection))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_ShowWithPlacement(This, selection, placement) \
    ((This)->lpVtbl->ShowWithPlacement(This, selection, placement))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_PickSingleDeviceAsync(This, selection, operation) \
    ((This)->lpVtbl->PickSingleDeviceAsync(This, selection, operation))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_PickSingleDeviceAsyncWithPlacement(This, selection, placement, operation) \
    ((This)->lpVtbl->PickSingleDeviceAsyncWithPlacement(This, selection, placement, operation))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_Hide(This) \
    ((This)->lpVtbl->Hide(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_SetDisplayStatus(This, device, status, options) \
    ((This)->lpVtbl->SetDisplayStatus(This, device, status, options))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePickerAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePickerAppearance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePickerAppearance[] = L"Windows.Devices.Enumeration.IDevicePickerAppearance";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_AccentColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_AccentColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedForegroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_SelectedForegroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedBackgroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_SelectedBackgroundColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedAccentColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_SelectedAccentColor)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance* This,
        struct __x_ABI_CWindows_CUI_CColor value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearanceVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_ForegroundColor(This, value) \
    ((This)->lpVtbl->put_ForegroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_AccentColor(This, value) \
    ((This)->lpVtbl->get_AccentColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_AccentColor(This, value) \
    ((This)->lpVtbl->put_AccentColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_SelectedForegroundColor(This, value) \
    ((This)->lpVtbl->get_SelectedForegroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_SelectedForegroundColor(This, value) \
    ((This)->lpVtbl->put_SelectedForegroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_SelectedBackgroundColor(This, value) \
    ((This)->lpVtbl->get_SelectedBackgroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_SelectedBackgroundColor(This, value) \
    ((This)->lpVtbl->put_SelectedBackgroundColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_get_SelectedAccentColor(This, value) \
    ((This)->lpVtbl->get_SelectedAccentColor(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_put_SelectedAccentColor(This, value) \
    ((This)->lpVtbl->put_SelectedAccentColor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDevicePickerFilter[] = L"Windows.Devices.Enumeration.IDevicePickerFilter";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedDeviceClasses)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        __FIVector_1_Windows__CDevices__CEnumeration__CDeviceClass** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedDeviceSelectors)(__x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilterVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_get_SupportedDeviceClasses(This, value) \
    ((This)->lpVtbl->get_SupportedDeviceClasses(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_get_SupportedDeviceSelectors(This, value) \
    ((This)->lpVtbl->get_SupportedDeviceSelectors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceSelectedEventArgs[] = L"Windows.Devices.Enumeration.IDeviceSelectedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedDevice)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_get_SelectedDevice(This, value) \
    ((This)->lpVtbl->get_SelectedDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceUnpairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceUnpairingResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceUnpairingResult[] = L"Windows.Devices.Enumeration.IDeviceUnpairingResult";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceUnpairingResultStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResultVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceUnpairingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcher[] = L"Windows.Devices.Enumeration.IDeviceWatcher";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformation* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_Windows__CDevices__CEnumeration__CDeviceInformationUpdate* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        __FITypedEventHandler_2_Windows__CDevices__CEnumeration__CDeviceWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherStatus* status);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcher2[] = L"Windows.Devices.Enumeration.IDeviceWatcher2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetBackgroundTrigger)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2* This,
        __FIIterable_1_Windows__CDevices__CEnumeration__CDeviceWatcherEventKind* requestedEventKinds,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIDeviceWatcherTrigger** trigger);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_GetBackgroundTrigger(This, requestedEventKinds, trigger) \
    ((This)->lpVtbl->GetBackgroundTrigger(This, requestedEventKinds, trigger))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcherEvent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcherEvent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcherEvent[] = L"Windows.Devices.Enumeration.IDeviceWatcherEvent";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEventVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceWatcherEventKind* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformationUpdate)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformationUpdate** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEventVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEventVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_get_DeviceInformationUpdate(This, value) \
    ((This)->lpVtbl->get_DeviceInformationUpdate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherEvent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.DeviceWatcherTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IDeviceWatcherTriggerDetails[] = L"Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceWatcherEvents)(__x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails* This,
        __FIVectorView_1_Windows__CDevices__CEnumeration__CDeviceWatcherEvent** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_get_DeviceWatcherEvents(This, value) \
    ((This)->lpVtbl->get_DeviceWatcherEvents(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceWatcherTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IEnclosureLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.EnclosureLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IEnclosureLocation[] = L"Windows.Devices.Enumeration.IEnclosureLocation";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InDock)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_InLid)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Panel)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocationVtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_get_InDock(This, value) \
    ((This)->lpVtbl->get_InDock(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_get_InLid(This, value) \
    ((This)->lpVtbl->get_InLid(This, value))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_get_Panel(This, value) \
    ((This)->lpVtbl->get_Panel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Enumeration.IEnclosureLocation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Enumeration.EnclosureLocation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Enumeration.IEnclosureLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Enumeration_IEnclosureLocation2[] = L"Windows.Devices.Enumeration.IEnclosureLocation2";
typedef struct __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RotationAngleInDegreesClockwise)(__x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2Vtbl;

interface __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_get_RotationAngleInDegreesClockwise(This, value) \
    ((This)->lpVtbl->get_RotationAngleInDegreesClockwise(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CEnumeration_CIEnclosureLocation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2
 *    Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceAccessChangedEventArgs[] = L"Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceAccessInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceAccessInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceAccessInformation ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceAccessInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceAccessInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceAccessInformation[] = L"Windows.Devices.Enumeration.DeviceAccessInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceConnectionChangeTriggerDetails[] = L"Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceDisconnectButtonClickedEventArgs[] = L"Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics3 interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformation ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformation[] = L"Windows.Devices.Enumeration.DeviceInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Enumeration.DeviceInformation> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Devices.Enumeration.DeviceInformation>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCollection_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationCollection[] = L"Windows.Devices.Enumeration.DeviceInformationCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationCustomPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationCustomPairing ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationCustomPairing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCustomPairing_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationCustomPairing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationCustomPairing[] = L"Windows.Devices.Enumeration.DeviceInformationCustomPairing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationPairing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationPairingStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Enumeration.IDeviceInformationPairingStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationPairing ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationPairing2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationPairing_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationPairing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationPairing[] = L"Windows.Devices.Enumeration.DeviceInformationPairing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceInformationUpdate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceInformationUpdate ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceInformationUpdate2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationUpdate_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceInformationUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceInformationUpdate[] = L"Windows.Devices.Enumeration.DeviceInformationUpdate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs ** Default Interface **
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2
 *    Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingRequestedEventArgs[] = L"Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingResult[] = L"Windows.Devices.Enumeration.DevicePairingResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePairingSetMembersRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePairingSetMembersRequestedEventArgs[] = L"Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePicker[] = L"Windows.Devices.Enumeration.DevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePickerAppearance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePickerAppearance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerAppearance_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerAppearance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePickerAppearance[] = L"Windows.Devices.Enumeration.DevicePickerAppearance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DevicePickerFilter[] = L"Windows.Devices.Enumeration.DevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceSelectedEventArgs[] = L"Windows.Devices.Enumeration.DeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceThumbnail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceThumbnail_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceThumbnail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceThumbnail[] = L"Windows.Devices.Enumeration.DeviceThumbnail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceUnpairingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceUnpairingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceUnpairingResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceUnpairingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceUnpairingResult[] = L"Windows.Devices.Enumeration.DeviceUnpairingResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcher ** Default Interface **
 *    Windows.Devices.Enumeration.IDeviceWatcher2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcher_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcher[] = L"Windows.Devices.Enumeration.DeviceWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcherEvent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcherEvent ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherEvent_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherEvent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcherEvent[] = L"Windows.Devices.Enumeration.DeviceWatcherEvent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.DeviceWatcherTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_DeviceWatcherTriggerDetails[] = L"Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Enumeration.EnclosureLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Enumeration.IEnclosureLocation ** Default Interface **
 *    Windows.Devices.Enumeration.IEnclosureLocation2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Enumeration_EnclosureLocation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Enumeration_EnclosureLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Enumeration_EnclosureLocation[] = L"Windows.Devices.Enumeration.EnclosureLocation";
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
#endif // __windows2Edevices2Eenumeration_p_h__

#endif // __windows2Edevices2Eenumeration_h__
