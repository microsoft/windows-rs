
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
#ifndef __windows2Emedia2Edevices_h__
#define __windows2Emedia2Edevices_h__
#ifndef __windows2Emedia2Edevices_p_h__
#define __windows2Emedia2Edevices_p_h__


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

#if !defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Media.Capture.h"
#include "Windows.Media.Devices.Core.h"
#include "Windows.Media.Effects.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICallControlEventHandler;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler ABI::Windows::Media::Devices::ICallControlEventHandler

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDialRequestedEventHandler;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler ABI::Windows::Media::Devices::IDialRequestedEventHandler

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IKeypadPressedEventHandler;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler ABI::Windows::Media::Devices::IKeypadPressedEventHandler

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IRedialRequestedEventHandler;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler ABI::Windows::Media::Devices::IRedialRequestedEventHandler

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedPhotoCaptureSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings ABI::Windows::Media::Devices::IAdvancedPhotoCaptureSettings

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedPhotoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl ABI::Windows::Media::Devices::IAdvancedPhotoControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController10;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController10

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController11;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController11

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController3;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController3

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController4;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController4

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController5;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController5

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController6;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController6

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController7;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController7

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController8;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController8

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAdvancedVideoCaptureDeviceController9;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9 ABI::Windows::Media::Devices::IAdvancedVideoCaptureDeviceController9

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController ABI::Windows::Media::Devices::IAudioDeviceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceController2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2 ABI::Windows::Media::Devices::IAudioDeviceController2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceModule;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule ABI::Windows::Media::Devices::IAudioDeviceModule

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceModuleNotificationEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs ABI::Windows::Media::Devices::IAudioDeviceModuleNotificationEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceModulesManager;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager ABI::Windows::Media::Devices::IAudioDeviceModulesManager

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IAudioDeviceModulesManagerFactory;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory ABI::Windows::Media::Devices::IAudioDeviceModulesManagerFactory

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICallControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICallControl ABI::Windows::Media::Devices::ICallControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICallControlStatics;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics ABI::Windows::Media::Devices::ICallControlStatics

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICameraOcclusionInfo;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo ABI::Windows::Media::Devices::ICameraOcclusionInfo

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICameraOcclusionState;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState ABI::Windows::Media::Devices::ICameraOcclusionState

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ICameraOcclusionStateChangedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs ABI::Windows::Media::Devices::ICameraOcclusionStateChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDefaultAudioDeviceChangedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs ABI::Windows::Media::Devices::IDefaultAudioDeviceChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDialRequestedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs ABI::Windows::Media::Devices::IDialRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDigitalWindowBounds;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds ABI::Windows::Media::Devices::IDigitalWindowBounds

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDigitalWindowCapability;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability ABI::Windows::Media::Devices::IDigitalWindowCapability

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IDigitalWindowControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl ABI::Windows::Media::Devices::IDigitalWindowControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IExposureCompensationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl ABI::Windows::Media::Devices::IExposureCompensationControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IExposureControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl ABI::Windows::Media::Devices::IExposureControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IExposurePriorityVideoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl ABI::Windows::Media::Devices::IExposurePriorityVideoControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IFlashControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl ABI::Windows::Media::Devices::IFlashControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IFlashControl2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2 ABI::Windows::Media::Devices::IFlashControl2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IFocusControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl ABI::Windows::Media::Devices::IFocusControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IFocusControl2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2 ABI::Windows::Media::Devices::IFocusControl2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IFocusSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings ABI::Windows::Media::Devices::IFocusSettings

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IHdrVideoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl ABI::Windows::Media::Devices::IHdrVideoControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IInfraredTorchControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl ABI::Windows::Media::Devices::IInfraredTorchControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IIsoSpeedControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl ABI::Windows::Media::Devices::IIsoSpeedControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IIsoSpeedControl2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2 ABI::Windows::Media::Devices::IIsoSpeedControl2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IKeypadPressedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs ABI::Windows::Media::Devices::IKeypadPressedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ILowLagPhotoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl ABI::Windows::Media::Devices::ILowLagPhotoControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ILowLagPhotoSequenceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl ABI::Windows::Media::Devices::ILowLagPhotoSequenceControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IMediaDeviceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl ABI::Windows::Media::Devices::IMediaDeviceControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IMediaDeviceControlCapabilities;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities ABI::Windows::Media::Devices::IMediaDeviceControlCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IMediaDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController ABI::Windows::Media::Devices::IMediaDeviceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IMediaDeviceStatics;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics ABI::Windows::Media::Devices::IMediaDeviceStatics

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IModuleCommandResult;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult ABI::Windows::Media::Devices::IModuleCommandResult

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IOpticalImageStabilizationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl ABI::Windows::Media::Devices::IOpticalImageStabilizationControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IPanelBasedOptimizationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl ABI::Windows::Media::Devices::IPanelBasedOptimizationControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IPhotoConfirmationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl ABI::Windows::Media::Devices::IPhotoConfirmationControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IRedialRequestedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs ABI::Windows::Media::Devices::IRedialRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IRegionOfInterest;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest ABI::Windows::Media::Devices::IRegionOfInterest

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IRegionOfInterest2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2 ABI::Windows::Media::Devices::IRegionOfInterest2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IRegionsOfInterestControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl ABI::Windows::Media::Devices::IRegionsOfInterestControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ISceneModeControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl ABI::Windows::Media::Devices::ISceneModeControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface ITorchControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl ABI::Windows::Media::Devices::ITorchControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IVideoDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController ABI::Windows::Media::Devices::IVideoDeviceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IVideoDeviceControllerGetDevicePropertyResult;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult ABI::Windows::Media::Devices::IVideoDeviceControllerGetDevicePropertyResult

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IVideoTemporalDenoisingControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl ABI::Windows::Media::Devices::IVideoTemporalDenoisingControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IWhiteBalanceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl ABI::Windows::Media::Devices::IWhiteBalanceControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IZoomControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl ABI::Windows::Media::Devices::IZoomControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IZoomControl2;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2 ABI::Windows::Media::Devices::IZoomControl2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                interface IZoomSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings ABI::Windows::Media::Devices::IZoomSettings

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ModuleCommandResult;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2e1f3d72-a58d-5b0a-b42d-3660c04cfeeb"))
IAsyncOperation<ABI::Windows::Media::Devices::ModuleCommandResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::ModuleCommandResult*, ABI::Windows::Media::Devices::IModuleCommandResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Devices.ModuleCommandResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Devices::ModuleCommandResult*> __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb786404-f2e8-5e0b-bf12-39e31483cfae"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Devices::ModuleCommandResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::ModuleCommandResult*, ABI::Windows::Media::Devices::IModuleCommandResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Devices.ModuleCommandResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Devices::ModuleCommandResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum AdvancedPhotoMode : int AdvancedPhotoMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e6d0bc9d-e1cb-5ed0-8ede-7d037bcc2e07"))
IIterator<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> : IIterator_impl<enum ABI::Windows::Media::Devices::AdvancedPhotoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.AdvancedPhotoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7d090784-70a9-570c-be82-0d0890318975"))
IIterable<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> : IIterable_impl<enum ABI::Windows::Media::Devices::AdvancedPhotoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.AdvancedPhotoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AudioDeviceModule;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b4cbbfb7-9851-56c9-839d-a10a8b1bb234"))
IIterator<ABI::Windows::Media::Devices::AudioDeviceModule*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::AudioDeviceModule*, ABI::Windows::Media::Devices::IAudioDeviceModule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.AudioDeviceModule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Devices::AudioDeviceModule*> __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7eeb51c3-d70e-548a-85c2-3cf71b4a124c"))
IIterable<ABI::Windows::Media::Devices::AudioDeviceModule*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::AudioDeviceModule*, ABI::Windows::Media::Devices::IAudioDeviceModule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.AudioDeviceModule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Devices::AudioDeviceModule*> __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum AutoFocusRange : int AutoFocusRange;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("07489ac5-3c71-59c6-b7dc-7f21341c2f71"))
IIterator<enum ABI::Windows::Media::Devices::AutoFocusRange> : IIterator_impl<enum ABI::Windows::Media::Devices::AutoFocusRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.AutoFocusRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::AutoFocusRange> __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("751664c6-f8d6-50a3-ab80-137c6d908c55"))
IIterable<enum ABI::Windows::Media::Devices::AutoFocusRange> : IIterable_impl<enum ABI::Windows::Media::Devices::AutoFocusRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.AutoFocusRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::AutoFocusRange> __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum CaptureSceneMode : int CaptureSceneMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aafa6d7a-2f7f-5dd7-aa0a-265731a2b3b3"))
IIterator<enum ABI::Windows::Media::Devices::CaptureSceneMode> : IIterator_impl<enum ABI::Windows::Media::Devices::CaptureSceneMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.CaptureSceneMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::CaptureSceneMode> __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("16d26b98-2cbc-52f0-ab64-1723714418e9"))
IIterable<enum ABI::Windows::Media::Devices::CaptureSceneMode> : IIterable_impl<enum ABI::Windows::Media::Devices::CaptureSceneMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.CaptureSceneMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::CaptureSceneMode> __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DigitalWindowCapability;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d1780c06-073e-5075-99e2-e12649268c03"))
IIterator<ABI::Windows::Media::Devices::DigitalWindowCapability*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::DigitalWindowCapability*, ABI::Windows::Media::Devices::IDigitalWindowCapability*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.DigitalWindowCapability>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Devices::DigitalWindowCapability*> __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a4f28d49-fb50-5c0d-ba48-6695e4612500"))
IIterable<ABI::Windows::Media::Devices::DigitalWindowCapability*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::DigitalWindowCapability*, ABI::Windows::Media::Devices::IDigitalWindowCapability*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.DigitalWindowCapability>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Devices::DigitalWindowCapability*> __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum FocusMode : int FocusMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f9a43cd4-b300-541f-af79-3de3400e16af"))
IIterator<enum ABI::Windows::Media::Devices::FocusMode> : IIterator_impl<enum ABI::Windows::Media::Devices::FocusMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.FocusMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::FocusMode> __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CFocusMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("561bc21f-4ae2-580a-a216-0ad48f373a4c"))
IIterable<enum ABI::Windows::Media::Devices::FocusMode> : IIterable_impl<enum ABI::Windows::Media::Devices::FocusMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.FocusMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::FocusMode> __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CFocusMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum FocusPreset : int FocusPreset;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3ebc8e9-f0c5-51c0-bb86-bdea0a6946fb"))
IIterator<enum ABI::Windows::Media::Devices::FocusPreset> : IIterator_impl<enum ABI::Windows::Media::Devices::FocusPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.FocusPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::FocusPreset> __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("26ba711b-3a32-5216-bc34-61ecafbebdc1"))
IIterable<enum ABI::Windows::Media::Devices::FocusPreset> : IIterable_impl<enum ABI::Windows::Media::Devices::FocusPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.FocusPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::FocusPreset> __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum HdrVideoMode : int HdrVideoMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3db61d13-0f30-5d2d-99cb-30c7b9009878"))
IIterator<enum ABI::Windows::Media::Devices::HdrVideoMode> : IIterator_impl<enum ABI::Windows::Media::Devices::HdrVideoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.HdrVideoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::HdrVideoMode> __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1d9679a7-2d06-5294-ac67-f9cd3561dcb8"))
IIterable<enum ABI::Windows::Media::Devices::HdrVideoMode> : IIterable_impl<enum ABI::Windows::Media::Devices::HdrVideoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.HdrVideoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::HdrVideoMode> __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum InfraredTorchMode : int InfraredTorchMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("42a9e83e-1786-57f4-906e-2f9b6f9f849a"))
IIterator<enum ABI::Windows::Media::Devices::InfraredTorchMode> : IIterator_impl<enum ABI::Windows::Media::Devices::InfraredTorchMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.InfraredTorchMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::InfraredTorchMode> __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("58a4b7b6-36c3-5541-b383-3690d7293c10"))
IIterable<enum ABI::Windows::Media::Devices::InfraredTorchMode> : IIterable_impl<enum ABI::Windows::Media::Devices::InfraredTorchMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.InfraredTorchMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::InfraredTorchMode> __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum IsoSpeedPreset : int IsoSpeedPreset;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b33af76-980b-5348-916a-793f61b555a0"))
IIterator<enum ABI::Windows::Media::Devices::IsoSpeedPreset> : IIterator_impl<enum ABI::Windows::Media::Devices::IsoSpeedPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.IsoSpeedPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::IsoSpeedPreset> __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94839abe-9712-545a-a94d-a567a3e8dfb7"))
IIterable<enum ABI::Windows::Media::Devices::IsoSpeedPreset> : IIterable_impl<enum ABI::Windows::Media::Devices::IsoSpeedPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.IsoSpeedPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::IsoSpeedPreset> __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum ManualFocusDistance : int ManualFocusDistance;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b02944e1-f649-511e-80dd-2e2b20379deb"))
IIterator<enum ABI::Windows::Media::Devices::ManualFocusDistance> : IIterator_impl<enum ABI::Windows::Media::Devices::ManualFocusDistance>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.ManualFocusDistance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::ManualFocusDistance> __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf8cbeb1-2a4c-522d-962f-84c31a598d68"))
IIterable<enum ABI::Windows::Media::Devices::ManualFocusDistance> : IIterable_impl<enum ABI::Windows::Media::Devices::ManualFocusDistance>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.ManualFocusDistance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::ManualFocusDistance> __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum OpticalImageStabilizationMode : int OpticalImageStabilizationMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4a165d46-cf19-5a03-bb54-63fc2b4ed39b"))
IIterator<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> : IIterator_impl<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.OpticalImageStabilizationMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("323d7734-94c2-544d-a560-56560fe68819"))
IIterable<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> : IIterable_impl<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.OpticalImageStabilizationMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class RegionOfInterest;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8eb80e4e-9691-594f-8b3d-f52ecc0f7837"))
IIterator<ABI::Windows::Media::Devices::RegionOfInterest*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::RegionOfInterest*, ABI::Windows::Media::Devices::IRegionOfInterest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.RegionOfInterest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Devices::RegionOfInterest*> __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d73144c7-9d75-5dfb-8040-626202dcf454"))
IIterable<ABI::Windows::Media::Devices::RegionOfInterest*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::RegionOfInterest*, ABI::Windows::Media::Devices::IRegionOfInterest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.RegionOfInterest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Devices::RegionOfInterest*> __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum VideoTemporalDenoisingMode : int VideoTemporalDenoisingMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9b062552-e75c-515e-a2a4-1b081b640614"))
IIterator<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> : IIterator_impl<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.VideoTemporalDenoisingMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("caf26629-ee84-5d4c-ae37-9dc4b26978eb"))
IIterable<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> : IIterable_impl<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.VideoTemporalDenoisingMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum ZoomTransitionMode : int ZoomTransitionMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("80eb468a-fdc4-5c89-99b8-8d476264e211"))
IIterator<enum ABI::Windows::Media::Devices::ZoomTransitionMode> : IIterator_impl<enum ABI::Windows::Media::Devices::ZoomTransitionMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.ZoomTransitionMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Media::Devices::ZoomTransitionMode> __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db656915-8fac-5fb2-98e0-0e97421656c5"))
IIterable<enum ABI::Windows::Media::Devices::ZoomTransitionMode> : IIterable_impl<enum ABI::Windows::Media::Devices::ZoomTransitionMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.ZoomTransitionMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Media::Devices::ZoomTransitionMode> __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties ABI::Windows::Media::MediaProperties::IMediaEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#define DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c094aec-c8f3-5f49-99c7-b66d8414200e"))
IIterator<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> : IIterator_impl<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.MediaProperties.IMediaEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t;
#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#define DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7fc75d5-3492-5bbb-9b34-dac3e24e79d0"))
IIterable<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> : IIterable_impl<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.MediaProperties.IMediaEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t;
#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d05843b6-03c8-523e-b6fb-1fcb03218a54"))
IVectorView<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::AdvancedPhotoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.AdvancedPhotoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::AdvancedPhotoMode> __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b9f55617-48ec-5ad7-95ca-33395284f28b"))
IVectorView<ABI::Windows::Media::Devices::AudioDeviceModule*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::AudioDeviceModule*, ABI::Windows::Media::Devices::IAudioDeviceModule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.AudioDeviceModule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Devices::AudioDeviceModule*> __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5df64825-c9e1-525f-9aeb-3c0f5f805f26"))
IVectorView<enum ABI::Windows::Media::Devices::AutoFocusRange> : IVectorView_impl<enum ABI::Windows::Media::Devices::AutoFocusRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.AutoFocusRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::AutoFocusRange> __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9b915d69-e718-5b48-bb94-54bdf3737ea5"))
IVectorView<enum ABI::Windows::Media::Devices::CaptureSceneMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::CaptureSceneMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.CaptureSceneMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::CaptureSceneMode> __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("18cb0c91-2753-542f-a33a-e4f9410f923b"))
IVectorView<ABI::Windows::Media::Devices::DigitalWindowCapability*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::DigitalWindowCapability*, ABI::Windows::Media::Devices::IDigitalWindowCapability*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.DigitalWindowCapability>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Devices::DigitalWindowCapability*> __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c877975c-f3c3-5b3d-93e4-5787be9b7f58"))
IVectorView<enum ABI::Windows::Media::Devices::FocusMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::FocusMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.FocusMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::FocusMode> __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d8a495e3-d7d4-5e9f-a9c2-6b250655a2e0"))
IVectorView<enum ABI::Windows::Media::Devices::FocusPreset> : IVectorView_impl<enum ABI::Windows::Media::Devices::FocusPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.FocusPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::FocusPreset> __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad0a4bbd-7630-53a4-aa9b-35a8c7d4958c"))
IVectorView<enum ABI::Windows::Media::Devices::HdrVideoMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::HdrVideoMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.HdrVideoMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::HdrVideoMode> __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b5af65de-bdf4-5155-b2b6-3139978e9c24"))
IVectorView<enum ABI::Windows::Media::Devices::InfraredTorchMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::InfraredTorchMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.InfraredTorchMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::InfraredTorchMode> __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ef6c9fe3-06f9-5eff-98a8-917d9644c946"))
IVectorView<enum ABI::Windows::Media::Devices::IsoSpeedPreset> : IVectorView_impl<enum ABI::Windows::Media::Devices::IsoSpeedPreset>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.IsoSpeedPreset>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::IsoSpeedPreset> __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("85f7663b-5467-5736-8d34-34395aa6d123"))
IVectorView<enum ABI::Windows::Media::Devices::ManualFocusDistance> : IVectorView_impl<enum ABI::Windows::Media::Devices::ManualFocusDistance>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.ManualFocusDistance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::ManualFocusDistance> __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c2658d8-acaa-5a80-b259-1ba0697c6138"))
IVectorView<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.OpticalImageStabilizationMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::OpticalImageStabilizationMode> __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("094d5da9-91cc-55d8-b7b7-52e597156987"))
IVectorView<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.VideoTemporalDenoisingMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::VideoTemporalDenoisingMode> __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4d556155-d021-5a46-9a1c-f401d61b8529"))
IVectorView<enum ABI::Windows::Media::Devices::ZoomTransitionMode> : IVectorView_impl<enum ABI::Windows::Media::Devices::ZoomTransitionMode>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.ZoomTransitionMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Media::Devices::ZoomTransitionMode> __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#define DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0f6c3b8b-5818-5cbf-bf26-6616bfc308c4"))
IVectorView<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> : IVectorView_impl<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.MediaProperties.IMediaEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::MediaProperties::IMediaEncodingProperties*> __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t;
#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#define DEF___FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0060b8a-1105-5ad4-963d-f6cf1905d349"))
IReference<enum ABI::Windows::Media::Devices::ManualFocusDistance> : IReference_impl<enum ABI::Windows::Media::Devices::ManualFocusDistance>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Media.Devices.ManualFocusDistance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Media::Devices::ManualFocusDistance> __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_t;
#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance ABI::Windows::Foundation::__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DefaultAudioCaptureDeviceChangedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("97d07327-2c78-57bc-98e6-a24cd024cf5b"))
ITypedEventHandler<IInspectable*, ABI::Windows::Media::Devices::DefaultAudioCaptureDeviceChangedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::DefaultAudioCaptureDeviceChangedEventArgs*, ABI::Windows::Media::Devices::IDefaultAudioDeviceChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::Media::Devices::DefaultAudioCaptureDeviceChangedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DefaultAudioRenderDeviceChangedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fd732aca-dafc-5b7d-bf72-b560b78d260c"))
ITypedEventHandler<IInspectable*, ABI::Windows::Media::Devices::DefaultAudioRenderDeviceChangedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::DefaultAudioRenderDeviceChangedEventArgs*, ABI::Windows::Media::Devices::IDefaultAudioDeviceChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::Media::Devices::DefaultAudioRenderDeviceChangedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AudioDeviceModulesManager;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AudioDeviceModuleNotificationEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b2f6b1fd-7092-5724-b2ce-91b1176e80e1"))
ITypedEventHandler<ABI::Windows::Media::Devices::AudioDeviceModulesManager*, ABI::Windows::Media::Devices::AudioDeviceModuleNotificationEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::AudioDeviceModulesManager*, ABI::Windows::Media::Devices::IAudioDeviceModulesManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::AudioDeviceModuleNotificationEventArgs*, ABI::Windows::Media::Devices::IAudioDeviceModuleNotificationEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Devices.AudioDeviceModulesManager, Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Devices::AudioDeviceModulesManager*, ABI::Windows::Media::Devices::AudioDeviceModuleNotificationEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class CameraOcclusionInfo;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class CameraOcclusionStateChangedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("71561417-e06e-54e7-b25e-39213c4e65d6"))
ITypedEventHandler<ABI::Windows::Media::Devices::CameraOcclusionInfo*, ABI::Windows::Media::Devices::CameraOcclusionStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::CameraOcclusionInfo*, ABI::Windows::Media::Devices::ICameraOcclusionInfo*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::CameraOcclusionStateChangedEventArgs*, ABI::Windows::Media::Devices::ICameraOcclusionStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Devices.CameraOcclusionInfo, Windows.Media.Devices.CameraOcclusionStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Devices::CameraOcclusionInfo*, ABI::Windows::Media::Devices::CameraOcclusionStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum Panel : int Panel;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

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
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                typedef enum MediaCaptureDeviceExclusiveControlReleaseMode : int MediaCaptureDeviceExclusiveControlReleaseMode;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                typedef enum MediaStreamType : int MediaStreamType;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                typedef enum PowerlineFrequency : int PowerlineFrequency;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class VariablePhotoSequenceController;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IVariablePhotoSequenceController;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController ABI::Windows::Media::Devices::Core::IVariablePhotoSequenceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AudioCaptureEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioCaptureEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager ABI::Windows::Media::Effects::IAudioCaptureEffectsManager

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum MediaPixelFormat : int MediaPixelFormat;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaRatio;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaRatio;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio ABI::Windows::Media::MediaProperties::IMediaRatio

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum MediaThumbnailFormat : int MediaThumbnailFormat;
            } /* MediaProperties */
        } /* Media */
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
        namespace Media {
            namespace Devices {
                typedef enum AudioDeviceRole : int AudioDeviceRole;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum CameraOcclusionKind : int CameraOcclusionKind;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum CaptureUse : int CaptureUse;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum ColorTemperaturePreset : int ColorTemperaturePreset;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum DigitalWindowMode : int DigitalWindowMode;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum MediaCaptureFocusState : int MediaCaptureFocusState;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum MediaCaptureOptimization : int MediaCaptureOptimization;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum RegionOfInterestType : int RegionOfInterestType;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum SendCommandStatus : int SendCommandStatus;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum TelephonyKey : int TelephonyKey;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum VideoDeviceControllerGetDevicePropertyStatus : int VideoDeviceControllerGetDevicePropertyStatus;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                typedef enum VideoDeviceControllerSetDevicePropertyStatus : int VideoDeviceControllerSetDevicePropertyStatus;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AdvancedPhotoCaptureSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AdvancedPhotoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class CallControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class CameraOcclusionState;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DialRequestedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DigitalWindowBounds;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class DigitalWindowControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ExposureCompensationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ExposureControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ExposurePriorityVideoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class FlashControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class FocusControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class FocusSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class HdrVideoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class InfraredTorchControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class IsoSpeedControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class KeypadPressedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class LowLagPhotoControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class LowLagPhotoSequenceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class MediaDeviceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class MediaDeviceControlCapabilities;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class OpticalImageStabilizationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class PanelBasedOptimizationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class PhotoConfirmationControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class RedialRequestedEventArgs;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class RegionsOfInterestControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class SceneModeControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class TorchControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class VideoDeviceControllerGetDevicePropertyResult;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class VideoTemporalDenoisingControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class WhiteBalanceControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ZoomControl;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class ZoomSettings;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Devices.AdvancedPhotoMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum AdvancedPhotoMode : int
                {
                    AdvancedPhotoMode_Auto = 0,
                    AdvancedPhotoMode_Standard = 1,
                    AdvancedPhotoMode_Hdr = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AdvancedPhotoMode_LowLight = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.AudioDeviceRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum AudioDeviceRole : int
                {
                    AudioDeviceRole_Default = 0,
                    AudioDeviceRole_Communications = 1,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.AutoFocusRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum AutoFocusRange : int
                {
                    AutoFocusRange_FullRange = 0,
                    AutoFocusRange_Macro = 1,
                    AutoFocusRange_Normal = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CameraOcclusionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum CameraOcclusionKind : int
                {
                    CameraOcclusionKind_Lid = 0,
                    CameraOcclusionKind_CameraHardware = 1,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Media.Devices.CameraStreamState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum CameraStreamState : int
                {
                    CameraStreamState_NotStreaming = 0,
                    CameraStreamState_Streaming = 1,
                    CameraStreamState_BlockedForPrivacy = 2,
                    CameraStreamState_Shutdown = 3,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CaptureSceneMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum CaptureSceneMode : int
                {
                    CaptureSceneMode_Auto = 0,
                    CaptureSceneMode_Manual = 1,
                    CaptureSceneMode_Macro = 2,
                    CaptureSceneMode_Portrait = 3,
                    CaptureSceneMode_Sport = 4,
                    CaptureSceneMode_Snow = 5,
                    CaptureSceneMode_Night = 6,
                    CaptureSceneMode_Beach = 7,
                    CaptureSceneMode_Sunset = 8,
                    CaptureSceneMode_Candlelight = 9,
                    CaptureSceneMode_Landscape = 10,
                    CaptureSceneMode_NightPortrait = 11,
                    CaptureSceneMode_Backlit = 12,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CaptureUse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum CaptureUse : int
                {
                    CaptureUse_None = 0,
                    CaptureUse_Photo = 1,
                    CaptureUse_Video = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.ColorTemperaturePreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum ColorTemperaturePreset : int
                {
                    ColorTemperaturePreset_Auto = 0,
                    ColorTemperaturePreset_Manual = 1,
                    ColorTemperaturePreset_Cloudy = 2,
                    ColorTemperaturePreset_Daylight = 3,
                    ColorTemperaturePreset_Flash = 4,
                    ColorTemperaturePreset_Fluorescent = 5,
                    ColorTemperaturePreset_Tungsten = 6,
                    ColorTemperaturePreset_Candlelight = 7,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.DigitalWindowMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum DigitalWindowMode : int
                {
                    DigitalWindowMode_Off = 0,
                    DigitalWindowMode_On = 1,
                    DigitalWindowMode_Auto = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Struct Windows.Media.Devices.FocusMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum FocusMode : int
                {
                    FocusMode_Auto = 0,
                    FocusMode_Single = 1,
                    FocusMode_Continuous = 2,
                    FocusMode_Manual = 3,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.FocusPreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum FocusPreset : int
                {
                    FocusPreset_Auto = 0,
                    FocusPreset_Manual = 1,
                    FocusPreset_AutoMacro = 2,
                    FocusPreset_AutoNormal = 3,
                    FocusPreset_AutoInfinity = 4,
                    FocusPreset_AutoHyperfocal = 5,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.HdrVideoMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum HdrVideoMode : int
                {
                    HdrVideoMode_Off = 0,
                    HdrVideoMode_On = 1,
                    HdrVideoMode_Auto = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.InfraredTorchMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum InfraredTorchMode : int
                {
                    InfraredTorchMode_Off = 0,
                    InfraredTorchMode_On = 1,
                    InfraredTorchMode_AlternatingFrameIllumination = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Devices.IsoSpeedPreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("IsoSpeedPreset may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IsoSpeedPreset : int
                {
                    IsoSpeedPreset_Auto = 0,
                    IsoSpeedPreset_Iso50 = 1,
                    IsoSpeedPreset_Iso80 = 2,
                    IsoSpeedPreset_Iso100 = 3,
                    IsoSpeedPreset_Iso200 = 4,
                    IsoSpeedPreset_Iso400 = 5,
                    IsoSpeedPreset_Iso800 = 6,
                    IsoSpeedPreset_Iso1600 = 7,
                    IsoSpeedPreset_Iso3200 = 8,
                    IsoSpeedPreset_Iso6400 = 9,
                    IsoSpeedPreset_Iso12800 = 10,
                    IsoSpeedPreset_Iso25600 = 11,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.ManualFocusDistance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum ManualFocusDistance : int
                {
                    ManualFocusDistance_Infinity = 0,
                    ManualFocusDistance_Hyperfocal = 1,
                    ManualFocusDistance_Nearest = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCaptureFocusState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum MediaCaptureFocusState : int
                {
                    MediaCaptureFocusState_Uninitialized = 0,
                    MediaCaptureFocusState_Lost = 1,
                    MediaCaptureFocusState_Searching = 2,
                    MediaCaptureFocusState_Focused = 3,
                    MediaCaptureFocusState_Failed = 4,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCaptureOptimization
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum MediaCaptureOptimization : int
                {
                    MediaCaptureOptimization_Default = 0,
                    MediaCaptureOptimization_Quality = 1,
                    MediaCaptureOptimization_Latency = 2,
                    MediaCaptureOptimization_Power = 3,
                    MediaCaptureOptimization_LatencyThenQuality = 4,
                    MediaCaptureOptimization_LatencyThenPower = 5,
                    MediaCaptureOptimization_PowerAndQuality = 6,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCapturePauseBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum MediaCapturePauseBehavior : int
                {
                    MediaCapturePauseBehavior_RetainHardwareResources = 0,
                    MediaCapturePauseBehavior_ReleaseHardwareResources = 1,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.OpticalImageStabilizationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum OpticalImageStabilizationMode : int
                {
                    OpticalImageStabilizationMode_Off = 0,
                    OpticalImageStabilizationMode_On = 1,
                    OpticalImageStabilizationMode_Auto = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.RegionOfInterestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum RegionOfInterestType : int
                {
                    RegionOfInterestType_Unknown = 0,
                    RegionOfInterestType_Face = 1,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.SendCommandStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum SendCommandStatus : int
                {
                    SendCommandStatus_Success = 0,
                    SendCommandStatus_DeviceNotAvailable = 1,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.TelephonyKey
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum TelephonyKey : int
                {
                    TelephonyKey_D0 = 0,
                    TelephonyKey_D1 = 1,
                    TelephonyKey_D2 = 2,
                    TelephonyKey_D3 = 3,
                    TelephonyKey_D4 = 4,
                    TelephonyKey_D5 = 5,
                    TelephonyKey_D6 = 6,
                    TelephonyKey_D7 = 7,
                    TelephonyKey_D8 = 8,
                    TelephonyKey_D9 = 9,
                    TelephonyKey_Star = 10,
                    TelephonyKey_Pound = 11,
                    TelephonyKey_A = 12,
                    TelephonyKey_B = 13,
                    TelephonyKey_C = 14,
                    TelephonyKey_D = 15,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum VideoDeviceControllerGetDevicePropertyStatus : int
                {
                    VideoDeviceControllerGetDevicePropertyStatus_Success = 0,
                    VideoDeviceControllerGetDevicePropertyStatus_UnknownFailure = 1,
                    VideoDeviceControllerGetDevicePropertyStatus_BufferTooSmall = 2,
                    VideoDeviceControllerGetDevicePropertyStatus_NotSupported = 3,
                    VideoDeviceControllerGetDevicePropertyStatus_DeviceNotAvailable = 4,
                    VideoDeviceControllerGetDevicePropertyStatus_MaxPropertyValueSizeTooSmall = 5,
                    VideoDeviceControllerGetDevicePropertyStatus_MaxPropertyValueSizeRequired = 6,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum VideoDeviceControllerSetDevicePropertyStatus : int
                {
                    VideoDeviceControllerSetDevicePropertyStatus_Success = 0,
                    VideoDeviceControllerSetDevicePropertyStatus_UnknownFailure = 1,
                    VideoDeviceControllerSetDevicePropertyStatus_NotSupported = 2,
                    VideoDeviceControllerSetDevicePropertyStatus_InvalidValue = 3,
                    VideoDeviceControllerSetDevicePropertyStatus_DeviceNotAvailable = 4,
                    VideoDeviceControllerSetDevicePropertyStatus_NotInControl = 5,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.VideoTemporalDenoisingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum VideoTemporalDenoisingMode : int
                {
                    VideoTemporalDenoisingMode_Off = 0,
                    VideoTemporalDenoisingMode_On = 1,
                    VideoTemporalDenoisingMode_Auto = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.Devices.ZoomTransitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                enum ZoomTransitionMode : int
                {
                    ZoomTransitionMode_Auto = 0,
                    ZoomTransitionMode_Direct = 1,
                    ZoomTransitionMode_Smooth = 2,
                };
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.CallControlEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("596f759f-50df-4454-bc63-4d3d01b61958")
                ICallControlEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Media::Devices::ICallControl* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallControlEventHandler = __uuidof(ICallControlEventHandler);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.DialRequestedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("5abbffdb-c21f-4bc4-891b-257e28c1b1a4")
                IDialRequestedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Media::Devices::ICallControl* sender,
                        ABI::Windows::Media::Devices::IDialRequestedEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialRequestedEventHandler = __uuidof(IDialRequestedEventHandler);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.KeypadPressedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("e637a454-c527-422c-8926-c9af83b559a0")
                IKeypadPressedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Media::Devices::ICallControl* sender,
                        ABI::Windows::Media::Devices::IKeypadPressedEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeypadPressedEventHandler = __uuidof(IKeypadPressedEventHandler);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.RedialRequestedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("baf257d1-4ebd-4b84-9f47-6ec43d75d8b1")
                IRedialRequestedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Media::Devices::ICallControl* sender,
                        ABI::Windows::Media::Devices::IRedialRequestedEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRedialRequestedEventHandler = __uuidof(IRedialRequestedEventHandler);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedPhotoCaptureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AdvancedPhotoCaptureSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedPhotoCaptureSettings[] = L"Windows.Media.Devices.IAdvancedPhotoCaptureSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("08f3863a-0018-445b-93d2-646d1c5ed05c")
                IAdvancedPhotoCaptureSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::AdvancedPhotoMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::AdvancedPhotoMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedPhotoCaptureSettings = __uuidof(IAdvancedPhotoCaptureSettings);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AdvancedPhotoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedPhotoControl[] = L"Windows.Media.Devices.IAdvancedPhotoControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c5b15486-9001-4682-9309-68eae0080eec")
                IAdvancedPhotoControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::AdvancedPhotoMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Configure(
                        ABI::Windows::Media::Devices::IAdvancedPhotoCaptureSettings* settings
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedPhotoControl = __uuidof(IAdvancedPhotoControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("de6ff4d3-2b96-4583-80ab-b5b01dc6a8d7")
                IAdvancedVideoCaptureDeviceController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetDeviceProperty(
                        HSTRING propertyId,
                        IInspectable* propertyValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceProperty(
                        HSTRING propertyId,
                        IInspectable** propertyValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController = __uuidof(IAdvancedVideoCaptureDeviceController);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController10[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c621b82d-d6f0-5c1b-a388-a6e938407146")
                IAdvancedVideoCaptureDeviceController10 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CameraOcclusionInfo(
                        ABI::Windows::Media::Devices::ICameraOcclusionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController10 = __uuidof(IAdvancedVideoCaptureDeviceController10);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController11[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("d5b65ae2-3772-580c-a630-e75de9106904")
                IAdvancedVideoCaptureDeviceController11 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryAcquireExclusiveControl(
                        HSTRING deviceId,
                        ABI::Windows::Media::Capture::MediaCaptureDeviceExclusiveControlReleaseMode mode,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController11 = __uuidof(IAdvancedVideoCaptureDeviceController11);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController2[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("8bb94f8f-f11a-43db-b402-11930b80ae56")
                IAdvancedVideoCaptureDeviceController2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LowLagPhotoSequence(
                        ABI::Windows::Media::Devices::ILowLagPhotoSequenceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LowLagPhoto(
                        ABI::Windows::Media::Devices::ILowLagPhotoControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SceneModeControl(
                        ABI::Windows::Media::Devices::ISceneModeControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TorchControl(
                        ABI::Windows::Media::Devices::ITorchControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FlashControl(
                        ABI::Windows::Media::Devices::IFlashControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WhiteBalanceControl(
                        ABI::Windows::Media::Devices::IWhiteBalanceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExposureControl(
                        ABI::Windows::Media::Devices::IExposureControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FocusControl(
                        ABI::Windows::Media::Devices::IFocusControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExposureCompensationControl(
                        ABI::Windows::Media::Devices::IExposureCompensationControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsoSpeedControl(
                        ABI::Windows::Media::Devices::IIsoSpeedControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RegionsOfInterestControl(
                        ABI::Windows::Media::Devices::IRegionsOfInterestControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryUse(
                        ABI::Windows::Media::Devices::CaptureUse* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrimaryUse(
                        ABI::Windows::Media::Devices::CaptureUse value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController2 = __uuidof(IAdvancedVideoCaptureDeviceController2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController3[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("a98b8f34-ee0d-470c-b9f0-4229c4bbd089")
                IAdvancedVideoCaptureDeviceController3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VariablePhotoSequenceController(
                        ABI::Windows::Media::Devices::Core::IVariablePhotoSequenceController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotoConfirmationControl(
                        ABI::Windows::Media::Devices::IPhotoConfirmationControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ZoomControl(
                        ABI::Windows::Media::Devices::IZoomControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController3 = __uuidof(IAdvancedVideoCaptureDeviceController3);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController4[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("ea9fbfaf-d371-41c3-9a17-824a87ebdfd2")
                IAdvancedVideoCaptureDeviceController4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExposurePriorityVideoControl(
                        ABI::Windows::Media::Devices::IExposurePriorityVideoControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredOptimization(
                        ABI::Windows::Media::Devices::MediaCaptureOptimization* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredOptimization(
                        ABI::Windows::Media::Devices::MediaCaptureOptimization value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HdrVideoControl(
                        ABI::Windows::Media::Devices::IHdrVideoControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OpticalImageStabilizationControl(
                        ABI::Windows::Media::Devices::IOpticalImageStabilizationControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdvancedPhotoControl(
                        ABI::Windows::Media::Devices::IAdvancedPhotoControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController4 = __uuidof(IAdvancedVideoCaptureDeviceController4);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController5[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("33512b17-b9cb-4a23-b875-f9eaab535492")
                IAdvancedVideoCaptureDeviceController5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDevicePropertyById(
                        HSTRING propertyId,
                        __FIReference_1_UINT32* maxPropertyValueSize,
                        ABI::Windows::Media::Devices::IVideoDeviceControllerGetDevicePropertyResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDevicePropertyById(
                        HSTRING propertyId,
                        IInspectable* propertyValue,
                        ABI::Windows::Media::Devices::VideoDeviceControllerSetDevicePropertyStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDevicePropertyByExtendedId(
                        UINT32 extendedPropertyIdLength,
                        BYTE* extendedPropertyId,
                        __FIReference_1_UINT32* maxPropertyValueSize,
                        ABI::Windows::Media::Devices::IVideoDeviceControllerGetDevicePropertyResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDevicePropertyByExtendedId(
                        UINT32 extendedPropertyIdLength,
                        BYTE* extendedPropertyId,
                        UINT32 propertyValueLength,
                        BYTE* propertyValue,
                        ABI::Windows::Media::Devices::VideoDeviceControllerSetDevicePropertyStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController5 = __uuidof(IAdvancedVideoCaptureDeviceController5);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController6[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("b6563a53-68a1-44b7-9f89-b5fa97ac0cbe")
                IAdvancedVideoCaptureDeviceController6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VideoTemporalDenoisingControl(
                        ABI::Windows::Media::Devices::IVideoTemporalDenoisingControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController6 = __uuidof(IAdvancedVideoCaptureDeviceController6);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController7[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("8d2927f0-a054-50e7-b7df-7c04234d10f0")
                IAdvancedVideoCaptureDeviceController7 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InfraredTorchControl(
                        ABI::Windows::Media::Devices::IInfraredTorchControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController7 = __uuidof(IAdvancedVideoCaptureDeviceController7);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController8[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("d843f010-e7fb-595b-9a78-0e54c4532b43")
                IAdvancedVideoCaptureDeviceController8 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PanelBasedOptimizationControl(
                        ABI::Windows::Media::Devices::IPanelBasedOptimizationControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController8 = __uuidof(IAdvancedVideoCaptureDeviceController8);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController9[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("8bdca95d-0255-51bc-a10d-5a169ec1625a")
                IAdvancedVideoCaptureDeviceController9 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DigitalWindowControl(
                        ABI::Windows::Media::Devices::IDigitalWindowControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedVideoCaptureDeviceController9 = __uuidof(IAdvancedVideoCaptureDeviceController9);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Devices.IMediaDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceController[] = L"Windows.Media.Devices.IAudioDeviceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("edd4a388-79c7-4f7c-90e8-ef934b21580a")
                IAudioDeviceController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Muted(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Muted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VolumePercent(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VolumePercent(
                        FLOAT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceController = __uuidof(IAudioDeviceController);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceController2[] = L"Windows.Media.Devices.IAudioDeviceController2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("85326599-4c24-48b0-81dd-0c5cc79ddf05")
                IAudioDeviceController2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AudioCaptureEffectsManager(
                        ABI::Windows::Media::Effects::IAudioCaptureEffectsManager** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceController2 = __uuidof(IAudioDeviceController2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModule[] = L"Windows.Media.Devices.IAudioDeviceModule";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("86cfac36-47c1-4b33-9852-8773ec4be123")
                IAudioDeviceModule : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ClassId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstanceId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MajorVersion(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinorVersion(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendCommandAsync(
                        ABI::Windows::Storage::Streams::IBuffer* Command,
                        __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceModule = __uuidof(IAudioDeviceModule);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModuleNotificationEventArgs[] = L"Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("e3e3ccaf-224c-48be-956b-9a13134e96e8")
                IAudioDeviceModuleNotificationEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Module(
                        ABI::Windows::Media::Devices::IAudioDeviceModule** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NotificationData(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceModuleNotificationEventArgs = __uuidof(IAudioDeviceModuleNotificationEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModulesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModulesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModulesManager[] = L"Windows.Media.Devices.IAudioDeviceModulesManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("6aa40c4d-960a-4d1c-b318-0022604547ed")
                IAudioDeviceModulesManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ModuleNotificationReceived(
                        __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ModuleNotificationReceived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllById(
                        HSTRING moduleId,
                        __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule** modules
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAll(
                        __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule** modules
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceModulesManager = __uuidof(IAudioDeviceModulesManager);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModulesManagerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModulesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModulesManagerFactory[] = L"Windows.Media.Devices.IAudioDeviceModulesManagerFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("8db03670-e64d-4773-96c0-bc7ebf0e063f")
                IAudioDeviceModulesManagerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING deviceId,
                        ABI::Windows::Media::Devices::IAudioDeviceModulesManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceModulesManagerFactory = __uuidof(IAudioDeviceModulesManagerFactory);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.ICallControl
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CallControl
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICallControl[] = L"Windows.Media.Devices.ICallControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("a520d0d6-ae8d-45db-8011-ca49d3b3e578")
                ICallControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IndicateNewIncomingCall(
                        boolean enableRinger,
                        HSTRING callerId,
                        UINT64* callToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IndicateNewOutgoingCall(
                        UINT64* callToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IndicateActiveCall(
                        UINT64 callToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EndCall(
                        UINT64 callToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasRinger(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AnswerRequested(
                        ABI::Windows::Media::Devices::ICallControlEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AnswerRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HangUpRequested(
                        ABI::Windows::Media::Devices::ICallControlEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HangUpRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DialRequested(
                        ABI::Windows::Media::Devices::IDialRequestedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DialRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RedialRequested(
                        ABI::Windows::Media::Devices::IRedialRequestedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RedialRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeypadPressed(
                        ABI::Windows::Media::Devices::IKeypadPressedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeypadPressed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AudioTransferRequested(
                        ABI::Windows::Media::Devices::ICallControlEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioTransferRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallControl = __uuidof(ICallControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ICallControlStatics
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CallControl
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICallControlStatics[] = L"Windows.Media.Devices.ICallControlStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("03945ad5-85ab-40e1-af19-56c94303b019")
                ICallControlStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Media::Devices::ICallControl** callControl
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING deviceId,
                        ABI::Windows::Media::Devices::ICallControl** callControl
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallControlStatics = __uuidof(ICallControlStatics);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControlStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionInfo[] = L"Windows.Media.Devices.ICameraOcclusionInfo";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("af6c4ad0-a84d-5db6-be58-a5da21cfe011")
                ICameraOcclusionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetState(
                        ABI::Windows::Media::Devices::ICameraOcclusionState** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsOcclusionKindSupported(
                        ABI::Windows::Media::Devices::CameraOcclusionKind occlusionKind,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICameraOcclusionInfo = __uuidof(ICameraOcclusionInfo);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionState[] = L"Windows.Media.Devices.ICameraOcclusionState";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("430adeb8-6842-5e55-9bde-04b4ef3a8a57")
                ICameraOcclusionState : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsOccluded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsOcclusionKind(
                        ABI::Windows::Media::Devices::CameraOcclusionKind occlusionKind,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICameraOcclusionState = __uuidof(ICameraOcclusionState);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionStateChangedEventArgs[] = L"Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("8512d848-c0de-57ca-a1ca-fb2c3d23df55")
                ICameraOcclusionStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::Devices::ICameraOcclusionState** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICameraOcclusionStateChangedEventArgs = __uuidof(ICameraOcclusionStateChangedEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDefaultAudioDeviceChangedEventArgs[] = L"Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("110f882f-1c05-4657-a18e-47c9b69f07ab")
                IDefaultAudioDeviceChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Role(
                        ABI::Windows::Media::Devices::AudioDeviceRole* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDefaultAudioDeviceChangedEventArgs = __uuidof(IDefaultAudioDeviceChangedEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IDialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DialRequestedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDialRequestedEventArgs[] = L"Windows.Media.Devices.IDialRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("037b929e-953c-4286-8866-4f0f376c855a")
                IDialRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Handled(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialRequestedEventArgs = __uuidof(IDialRequestedEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowBounds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowBounds[] = L"Windows.Media.Devices.IDigitalWindowBounds";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("dd4f21dd-d173-5c6b-8c25-bdd26d5122b1")
                IDigitalWindowBounds : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NormalizedOriginTop(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NormalizedOriginTop(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NormalizedOriginLeft(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NormalizedOriginLeft(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Scale(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Scale(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDigitalWindowBounds = __uuidof(IDigitalWindowBounds);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowCapability[] = L"Windows.Media.Devices.IDigitalWindowCapability";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("d78bad2c-f721-5244-a196-b56ccbec606c")
                IDigitalWindowCapability : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinScaleValue(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxScaleValue(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinScaleValueWithoutUpsampling(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NormalizedFieldOfViewLimit(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDigitalWindowCapability = __uuidof(IDigitalWindowCapability);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowControl[] = L"Windows.Media.Devices.IDigitalWindowControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("23b69eff-65d2-53ea-8780-de582b48b544")
                IDigitalWindowControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        UINT32* valueLength,
                        ABI::Windows::Media::Devices::DigitalWindowMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentMode(
                        ABI::Windows::Media::Devices::DigitalWindowMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBounds(
                        ABI::Windows::Media::Devices::IDigitalWindowBounds** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Configure(
                        ABI::Windows::Media::Devices::DigitalWindowMode digitalWindowMode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConfigureWithBounds(
                        ABI::Windows::Media::Devices::DigitalWindowMode digitalWindowMode,
                        ABI::Windows::Media::Devices::IDigitalWindowBounds* digitalWindowBounds
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedCapabilities(
                        __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCapabilityForSize(
                        INT32 width,
                        INT32 height,
                        ABI::Windows::Media::Devices::IDigitalWindowCapability** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDigitalWindowControl = __uuidof(IDigitalWindowControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposureCompensationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposureCompensationControl[] = L"Windows.Media.Devices.IExposureCompensationControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("81c8e834-dcec-4011-a610-1f3847e64aca")
                IExposureCompensationControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        FLOAT value,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IExposureCompensationControl = __uuidof(IExposureCompensationControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposureControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposureControl[] = L"Windows.Media.Devices.IExposureControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("09e8cbe2-ad96-4f28-a0e0-96ed7e1b5fd2")
                IExposureControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Auto(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetAutoAsync(
                        boolean value,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        ABI::Windows::Foundation::TimeSpan shutterDuration,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IExposureControl = __uuidof(IExposureControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposureControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IExposurePriorityVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposurePriorityVideoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposurePriorityVideoControl[] = L"Windows.Media.Devices.IExposurePriorityVideoControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("2cb240a3-5168-4271-9ea5-47621a98a352")
                IExposurePriorityVideoControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IExposurePriorityVideoControl = __uuidof(IExposurePriorityVideoControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFlashControl[] = L"Windows.Media.Devices.IFlashControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("def41dbe-7d68-45e3-8c0f-be7bb32837d0")
                IFlashControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RedEyeReductionSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Auto(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Auto(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RedEyeReduction(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RedEyeReduction(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerPercent(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PowerPercent(
                        FLOAT value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFlashControl = __uuidof(IFlashControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFlashControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFlashControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFlashControl2[] = L"Windows.Media.Devices.IFlashControl2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("7d29cc9e-75e1-4af7-bd7d-4e38e1c06cd6")
                IFlashControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AssistantLightSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AssistantLightEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AssistantLightEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFlashControl2 = __uuidof(IFlashControl2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFlashControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusControl[] = L"Windows.Media.Devices.IFocusControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c0d889f6-5228-4453-b153-85606592b238")
                IFocusControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedPresets(
                        __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Preset(
                        ABI::Windows::Media::Devices::FocusPreset* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPresetAsync(
                        ABI::Windows::Media::Devices::FocusPreset preset,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPresetWithCompletionOptionAsync(
                        ABI::Windows::Media::Devices::FocusPreset preset,
                        boolean completeBeforeFocus,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        UINT32 focus,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FocusAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusControl = __uuidof(IFocusControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusControl2[] = L"Windows.Media.Devices.IFocusControl2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("3f7cff48-c534-4e9e-94c3-52ef2afd5d07")
                IFocusControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FocusChangedSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WaitForFocusSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedFocusModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedFocusDistances(
                        __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedFocusRanges(
                        __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::FocusMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FocusState(
                        ABI::Windows::Media::Devices::MediaCaptureFocusState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnlockAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LockAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Configure(
                        ABI::Windows::Media::Devices::IFocusSettings* settings
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusControl2 = __uuidof(IFocusControl2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusSettings[] = L"Windows.Media.Devices.IFocusSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("79958f6b-3263-4275-85d6-aeae891c96ee")
                IFocusSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::FocusMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::FocusMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoFocusRange(
                        ABI::Windows::Media::Devices::AutoFocusRange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoFocusRange(
                        ABI::Windows::Media::Devices::AutoFocusRange value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        __FIReference_1_UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Distance(
                        __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Distance(
                        __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WaitForFocus(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WaitForFocus(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisableDriverFallback(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisableDriverFallback(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFocusSettings = __uuidof(IFocusSettings);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IHdrVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.HdrVideoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IHdrVideoControl[] = L"Windows.Media.Devices.IHdrVideoControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("55d8e2d0-30c0-43bf-9b9a-9799d70ced94")
                IHdrVideoControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::HdrVideoMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::HdrVideoMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHdrVideoControl = __uuidof(IHdrVideoControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IInfraredTorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.InfraredTorchControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IInfraredTorchControl[] = L"Windows.Media.Devices.IInfraredTorchControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("1cba2c83-6cb6-5a04-a6fc-3be7b33ff056")
                IInfraredTorchControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentMode(
                        ABI::Windows::Media::Devices::InfraredTorchMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CurrentMode(
                        ABI::Windows::Media::Devices::InfraredTorchMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinPower(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPower(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerStep(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Power(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Power(
                        INT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInfraredTorchControl = __uuidof(IInfraredTorchControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Devices.IIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.IsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IIsoSpeedControl[] = L"Windows.Media.Devices.IIsoSpeedControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("27b6c322-25ad-4f1b-aaab-524ab376ca33")
                IIsoSpeedControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SupportedPresets may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedPresets(
                        __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Preset may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Preset(
                        ABI::Windows::Media::Devices::IsoSpeedPreset* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SetPresetAsync may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetPresetAsync(
                        ABI::Windows::Media::Devices::IsoSpeedPreset preset,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsoSpeedControl = __uuidof(IIsoSpeedControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IIsoSpeedControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.IsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IIsoSpeedControl2[] = L"Windows.Media.Devices.IIsoSpeedControl2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("6f1578f2-6d77-4f8a-8c2f-6130b6395053")
                IIsoSpeedControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        UINT32 isoSpeed,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Auto(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetAutoAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsoSpeedControl2 = __uuidof(IIsoSpeedControl2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IKeypadPressedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.KeypadPressedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IKeypadPressedEventArgs[] = L"Windows.Media.Devices.IKeypadPressedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("d3a43900-b4fa-49cd-9442-89af6568f601")
                IKeypadPressedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TelephonyKey(
                        ABI::Windows::Media::Devices::TelephonyKey* telephonyKey
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeypadPressedEventArgs = __uuidof(IKeypadPressedEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ILowLagPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.LowLagPhotoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ILowLagPhotoControl[] = L"Windows.Media.Devices.ILowLagPhotoControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("6d5c4dd0-fadf-415d-aee6-3baa529300c9")
                ILowLagPhotoControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetHighestConcurrentFrameRate(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties* captureProperties,
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentFrameRate(
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThumbnailEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ThumbnailEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThumbnailFormat(
                        ABI::Windows::Media::MediaProperties::MediaThumbnailFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ThumbnailFormat(
                        ABI::Windows::Media::MediaProperties::MediaThumbnailFormat value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredThumbnailSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredThumbnailSize(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareAcceleratedThumbnailSupported(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILowLagPhotoControl = __uuidof(ILowLagPhotoControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ILowLagPhotoSequenceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.LowLagPhotoSequenceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ILowLagPhotoSequenceControl[] = L"Windows.Media.Devices.ILowLagPhotoSequenceControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("3dcf909d-6d16-409c-bafe-b9a594c6fde6")
                ILowLagPhotoSequenceControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPastPhotos(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPhotosPerSecond(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PastPhotoLimit(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PastPhotoLimit(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosPerSecondLimit(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PhotosPerSecondLimit(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHighestConcurrentFrameRate(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties* captureProperties,
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentFrameRate(
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThumbnailEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ThumbnailEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThumbnailFormat(
                        ABI::Windows::Media::MediaProperties::MediaThumbnailFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ThumbnailFormat(
                        ABI::Windows::Media::MediaProperties::MediaThumbnailFormat value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredThumbnailSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredThumbnailSize(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareAcceleratedThumbnailSupported(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILowLagPhotoSequenceControl = __uuidof(ILowLagPhotoSequenceControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDeviceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceControl[] = L"Windows.Media.Devices.IMediaDeviceControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("efa8dfa9-6f75-4863-ba0b-583f3036b4de")
                IMediaDeviceControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Capabilities(
                        ABI::Windows::Media::Devices::IMediaDeviceControlCapabilities** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetValue(
                        DOUBLE* value,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetValue(
                        DOUBLE value,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetAuto(
                        boolean* value,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetAuto(
                        boolean value,
                        boolean* succeeded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaDeviceControl = __uuidof(IMediaDeviceControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDeviceControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceControlCapabilities[] = L"Windows.Media.Devices.IMediaDeviceControlCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("23005816-eb85-43e2-b92b-8240d5ee70ec")
                IMediaDeviceControlCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Default(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoModeSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaDeviceControlCapabilities = __uuidof(IMediaDeviceControlCapabilities);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceController[] = L"Windows.Media.Devices.IMediaDeviceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("f6f8f5ce-209a-48fb-86fc-d44578f317e6")
                IMediaDeviceController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAvailableMediaStreamProperties(
                        ABI::Windows::Media::Capture::MediaStreamType mediaStreamType,
                        __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMediaStreamProperties(
                        ABI::Windows::Media::Capture::MediaStreamType mediaStreamType,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetMediaStreamPropertiesAsync(
                        ABI::Windows::Media::Capture::MediaStreamType mediaStreamType,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties* mediaEncodingProperties,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaDeviceController = __uuidof(IMediaDeviceController);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceStatics[] = L"Windows.Media.Devices.IMediaDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("aa2d9a40-909f-4bba-bf8b-0c0d296f14f0")
                IMediaDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAudioCaptureSelector(
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioRenderSelector(
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVideoCaptureSelector(
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAudioCaptureId(
                        ABI::Windows::Media::Devices::AudioDeviceRole role,
                        HSTRING* deviceId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAudioRenderId(
                        ABI::Windows::Media::Devices::AudioDeviceRole role,
                        HSTRING* deviceId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DefaultAudioCaptureDeviceChanged(
                        __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DefaultAudioCaptureDeviceChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DefaultAudioRenderDeviceChanged(
                        __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DefaultAudioRenderDeviceChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaDeviceStatics = __uuidof(IMediaDeviceStatics);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IModuleCommandResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ModuleCommandResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IModuleCommandResult[] = L"Windows.Media.Devices.IModuleCommandResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("520d1eb4-1374-4c7d-b1e4-39dcdf3eae4e")
                IModuleCommandResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Devices::SendCommandStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IModuleCommandResult = __uuidof(IModuleCommandResult);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IOpticalImageStabilizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.OpticalImageStabilizationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IOpticalImageStabilizationControl[] = L"Windows.Media.Devices.IOpticalImageStabilizationControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("bfad9c1d-00bc-423b-8eb2-a0178ca94247")
                IOpticalImageStabilizationControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::OpticalImageStabilizationMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::OpticalImageStabilizationMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOpticalImageStabilizationControl = __uuidof(IOpticalImageStabilizationControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IPanelBasedOptimizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.PanelBasedOptimizationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IPanelBasedOptimizationControl[] = L"Windows.Media.Devices.IPanelBasedOptimizationControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("33323223-6247-5419-a5a4-3d808645d917")
                IPanelBasedOptimizationControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Panel(
                        ABI::Windows::Devices::Enumeration::Panel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Panel(
                        ABI::Windows::Devices::Enumeration::Panel value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPanelBasedOptimizationControl = __uuidof(IPanelBasedOptimizationControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Devices.IPhotoConfirmationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.PhotoConfirmationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IPhotoConfirmationControl[] = L"Windows.Media.Devices.IPhotoConfirmationControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c8f3f363-ff5e-4582-a9a8-0550f85a4a76")
                IPhotoConfirmationControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* pbSupported
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelFormat(
                        ABI::Windows::Media::MediaProperties::MediaPixelFormat* format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PixelFormat(
                        ABI::Windows::Media::MediaProperties::MediaPixelFormat format
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoConfirmationControl = __uuidof(IPhotoConfirmationControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRedialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RedialRequestedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRedialRequestedEventArgs[] = L"Windows.Media.Devices.IRedialRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("7eb55209-76ab-4c31-b40e-4b58379d580c")
                IRedialRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Handled(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IRedialRequestedEventArgs = __uuidof(IRedialRequestedEventArgs);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionOfInterest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionOfInterest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionOfInterest[] = L"Windows.Media.Devices.IRegionOfInterest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("e5ecc834-ce66-4e05-a78f-cf391a5ec2d1")
                IRegionOfInterest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AutoFocusEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoFocusEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoWhiteBalanceEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoWhiteBalanceEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoExposureEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoExposureEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Bounds(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRegionOfInterest = __uuidof(IRegionOfInterest);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionOfInterest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionOfInterest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionOfInterest2[] = L"Windows.Media.Devices.IRegionOfInterest2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("19fe2a91-73aa-4d51-8a9d-56ccf7db7f54")
                IRegionOfInterest2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Media::Devices::RegionOfInterestType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Type(
                        ABI::Windows::Media::Devices::RegionOfInterestType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BoundsNormalized(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BoundsNormalized(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Weight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Weight(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRegionOfInterest2 = __uuidof(IRegionOfInterest2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionsOfInterestControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionsOfInterestControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionsOfInterestControl[] = L"Windows.Media.Devices.IRegionsOfInterestControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c323f527-ab0b-4558-8b5b-df5693db0378")
                IRegionsOfInterestControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxRegions(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetRegionsAsync(
                        __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* regions,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetRegionsWithLockAsync(
                        __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* regions,
                        boolean lockValues,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearRegionsAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoFocusSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoWhiteBalanceSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoExposureSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRegionsOfInterestControl = __uuidof(IRegionsOfInterestControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ISceneModeControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.SceneModeControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ISceneModeControl[] = L"Windows.Media.Devices.ISceneModeControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("d48e5af7-8d59-4854-8c62-12c70ba89b7c")
                ISceneModeControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Media::Devices::CaptureSceneMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        ABI::Windows::Media::Devices::CaptureSceneMode sceneMode,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISceneModeControl = __uuidof(ISceneModeControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CISceneModeControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ITorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.TorchControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ITorchControl[] = L"Windows.Media.Devices.ITorchControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("a6053665-8250-416c-919a-724296afa306")
                ITorchControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerPercent(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PowerPercent(
                        FLOAT value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITorchControl = __uuidof(ITorchControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CITorchControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IVideoDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Devices.IMediaDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoDeviceController[] = L"Windows.Media.Devices.IVideoDeviceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("99555575-2e2e-40b8-b6c7-f82d10013210")
                IVideoDeviceController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Brightness(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contrast(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hue(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WhiteBalance(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BacklightCompensation(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pan(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tilt(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Zoom(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Roll(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Exposure(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Focus(
                        ABI::Windows::Media::Devices::IMediaDeviceControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetPowerlineFrequency(
                        ABI::Windows::Media::Capture::PowerlineFrequency value,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetPowerlineFrequency(
                        ABI::Windows::Media::Capture::PowerlineFrequency* value,
                        boolean* succeeded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoDeviceController = __uuidof(IVideoDeviceController);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoDeviceControllerGetDevicePropertyResult[] = L"Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("c5d88395-6ed5-4790-8b5d-0ef13935d0f8")
                IVideoDeviceControllerGetDevicePropertyResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Devices::VideoDeviceControllerGetDevicePropertyStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoDeviceControllerGetDevicePropertyResult = __uuidof(IVideoDeviceControllerGetDevicePropertyResult);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IVideoTemporalDenoisingControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoTemporalDenoisingControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoTemporalDenoisingControl[] = L"Windows.Media.Devices.IVideoTemporalDenoisingControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("7ab34735-3e2a-4a32-baff-4358c4fbdd57")
                IVideoTemporalDenoisingControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::VideoTemporalDenoisingMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::VideoTemporalDenoisingMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoTemporalDenoisingControl = __uuidof(IVideoTemporalDenoisingControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Devices.IWhiteBalanceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.WhiteBalanceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IWhiteBalanceControl[] = L"Windows.Media.Devices.IWhiteBalanceControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("781f047e-7162-49c8-a8f9-9481c565363e")
                IWhiteBalanceControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Preset(
                        ABI::Windows::Media::Devices::ColorTemperaturePreset* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPresetAsync(
                        ABI::Windows::Media::Devices::ColorTemperaturePreset preset,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetValueAsync(
                        UINT32 temperature,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWhiteBalanceControl = __uuidof(IWhiteBalanceControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomControl[] = L"Windows.Media.Devices.IZoomControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("3a1e0b12-32da-4c17-bfd7-8d0c73c8f5a5")
                IZoomControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Supported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Min(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Max(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Step(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        FLOAT value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IZoomControl = __uuidof(IZoomControl);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomControl2[] = L"Windows.Media.Devices.IZoomControl2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("69843db0-2e99-4641-8529-184f319d1671")
                IZoomControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModes(
                        __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::ZoomTransitionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Configure(
                        ABI::Windows::Media::Devices::IZoomSettings* settings
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IZoomControl2 = __uuidof(IZoomControl2);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomSettings[] = L"Windows.Media.Devices.IZoomSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                MIDL_INTERFACE("6ad66b24-14b4-4bfd-b18f-88fe24463b52")
                IZoomSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Devices::ZoomTransitionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Devices::ZoomTransitionMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        FLOAT value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IZoomSettings = __uuidof(IZoomSettings);
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AdvancedPhotoCaptureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAdvancedPhotoCaptureSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoCaptureSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoCaptureSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AdvancedPhotoCaptureSettings[] = L"Windows.Media.Devices.AdvancedPhotoCaptureSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AdvancedPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAdvancedPhotoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AdvancedPhotoControl[] = L"Windows.Media.Devices.AdvancedPhotoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceController ** Default Interface **
 *    Windows.Media.Devices.IMediaDeviceController
 *    Windows.Media.Devices.IAudioDeviceController2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceController[] = L"Windows.Media.Devices.AudioDeviceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModule ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModule_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModule[] = L"Windows.Media.Devices.AudioDeviceModule";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs[] = L"Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModulesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Devices.IAudioDeviceModulesManagerFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModulesManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModulesManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModulesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModulesManager[] = L"Windows.Media.Devices.AudioDeviceModulesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.CallControl
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Devices.ICallControlStatics interface starting with version 1.0 of the Windows.Media.Devices.CallControlContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICallControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CallControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CallControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CallControl[] = L"Windows.Media.Devices.CallControl";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionInfo[] = L"Windows.Media.Devices.CameraOcclusionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionState_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionState[] = L"Windows.Media.Devices.CameraOcclusionState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs[] = L"Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs[] = L"Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs[] = L"Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDialRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DialRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DialRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DialRequestedEventArgs[] = L"Windows.Media.Devices.DialRequestedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowBounds ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowBounds_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowBounds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowBounds[] = L"Windows.Media.Devices.DigitalWindowBounds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowCapability ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowCapability_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowCapability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowCapability[] = L"Windows.Media.Devices.DigitalWindowCapability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowControl[] = L"Windows.Media.Devices.DigitalWindowControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.ExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposureCompensationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposureCompensationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposureCompensationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposureCompensationControl[] = L"Windows.Media.Devices.ExposureCompensationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposureControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposureControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposureControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposureControl[] = L"Windows.Media.Devices.ExposureControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ExposurePriorityVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposurePriorityVideoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposurePriorityVideoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposurePriorityVideoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposurePriorityVideoControl[] = L"Windows.Media.Devices.ExposurePriorityVideoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFlashControl ** Default Interface **
 *    Windows.Media.Devices.IFlashControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FlashControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FlashControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FlashControl[] = L"Windows.Media.Devices.FlashControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFocusControl ** Default Interface **
 *    Windows.Media.Devices.IFocusControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FocusControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FocusControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FocusControl[] = L"Windows.Media.Devices.FocusControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FocusSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFocusSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FocusSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FocusSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FocusSettings[] = L"Windows.Media.Devices.FocusSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.HdrVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IHdrVideoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_HdrVideoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_HdrVideoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_HdrVideoControl[] = L"Windows.Media.Devices.HdrVideoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.InfraredTorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IInfraredTorchControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Devices_InfraredTorchControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_InfraredTorchControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_InfraredTorchControl[] = L"Windows.Media.Devices.InfraredTorchControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Devices.IsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IIsoSpeedControl ** Default Interface **
 *    Windows.Media.Devices.IIsoSpeedControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_IsoSpeedControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_IsoSpeedControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_IsoSpeedControl[] = L"Windows.Media.Devices.IsoSpeedControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.KeypadPressedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IKeypadPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_KeypadPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_KeypadPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_KeypadPressedEventArgs[] = L"Windows.Media.Devices.KeypadPressedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.LowLagPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ILowLagPhotoControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_LowLagPhotoControl[] = L"Windows.Media.Devices.LowLagPhotoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.LowLagPhotoSequenceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ILowLagPhotoSequenceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoSequenceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoSequenceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_LowLagPhotoSequenceControl[] = L"Windows.Media.Devices.LowLagPhotoSequenceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Devices.IMediaDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDevice[] = L"Windows.Media.Devices.MediaDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDeviceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IMediaDeviceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDeviceControl[] = L"Windows.Media.Devices.MediaDeviceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDeviceControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IMediaDeviceControlCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControlCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControlCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDeviceControlCapabilities[] = L"Windows.Media.Devices.MediaDeviceControlCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ModuleCommandResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IModuleCommandResult ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ModuleCommandResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ModuleCommandResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ModuleCommandResult[] = L"Windows.Media.Devices.ModuleCommandResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.OpticalImageStabilizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IOpticalImageStabilizationControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_OpticalImageStabilizationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_OpticalImageStabilizationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_OpticalImageStabilizationControl[] = L"Windows.Media.Devices.OpticalImageStabilizationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.PanelBasedOptimizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IPanelBasedOptimizationControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_PanelBasedOptimizationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_PanelBasedOptimizationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_PanelBasedOptimizationControl[] = L"Windows.Media.Devices.PanelBasedOptimizationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Devices.PhotoConfirmationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IPhotoConfirmationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_PhotoConfirmationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_PhotoConfirmationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_PhotoConfirmationControl[] = L"Windows.Media.Devices.PhotoConfirmationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RedialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRedialRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RedialRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RedialRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RedialRequestedEventArgs[] = L"Windows.Media.Devices.RedialRequestedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RegionOfInterest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRegionOfInterest ** Default Interface **
 *    Windows.Media.Devices.IRegionOfInterest2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RegionOfInterest_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RegionOfInterest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RegionOfInterest[] = L"Windows.Media.Devices.RegionOfInterest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RegionsOfInterestControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRegionsOfInterestControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RegionsOfInterestControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RegionsOfInterestControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RegionsOfInterestControl[] = L"Windows.Media.Devices.RegionsOfInterestControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.SceneModeControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ISceneModeControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_SceneModeControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_SceneModeControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_SceneModeControl[] = L"Windows.Media.Devices.SceneModeControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.TorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ITorchControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_TorchControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_TorchControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_TorchControl[] = L"Windows.Media.Devices.TorchControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.VideoDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoDeviceController ** Default Interface **
 *    Windows.Media.Devices.IMediaDeviceController
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoDeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoDeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoDeviceController[] = L"Windows.Media.Devices.VideoDeviceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult[] = L"Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.VideoTemporalDenoisingControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoTemporalDenoisingControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoTemporalDenoisingControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoTemporalDenoisingControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoTemporalDenoisingControl[] = L"Windows.Media.Devices.VideoTemporalDenoisingControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Devices.WhiteBalanceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IWhiteBalanceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_WhiteBalanceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_WhiteBalanceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_WhiteBalanceControl[] = L"Windows.Media.Devices.WhiteBalanceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ZoomControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IZoomControl ** Default Interface **
 *    Windows.Media.Devices.IZoomControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ZoomControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ZoomControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ZoomControl[] = L"Windows.Media.Devices.ZoomControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ZoomSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IZoomSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ZoomSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ZoomSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ZoomSettings[] = L"Windows.Media.Devices.ZoomSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9 __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2 __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICallControl __x_ABI_CWindows_CMedia_CDevices_CICallControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIExposureControl __x_ABI_CWindows_CMedia_CDevices_CIExposureControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIFlashControl __x_ABI_CWindows_CMedia_CDevices_CIFlashControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2 __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIFocusControl __x_ABI_CWindows_CMedia_CDevices_CIFocusControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2 __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2 __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2 __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CITorchControl __x_ABI_CWindows_CMedia_CDevices_CITorchControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIZoomControl __x_ABI_CWindows_CMedia_CDevices_CIZoomControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2 __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDevices__CModuleCommandResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CAdvancedPhotoMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        __FIIterator_1_Windows__CMedia__CDevices__CAudioDeviceModule** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        __FIIterator_1_Windows__CMedia__CDevices__CAutoFocusRange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CCaptureSceneMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        __FIIterator_1_Windows__CMedia__CDevices__CDigitalWindowCapability** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode __x_ABI_CWindows_CMedia_CDevices_CFocusMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CFocusMode __FIIterator_1_Windows__CMedia__CDevices__CFocusMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CFocusMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CFocusModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CFocusMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CFocusModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CFocusMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CFocusModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CFocusMode __FIIterable_1_Windows__CMedia__CDevices__CFocusMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CFocusMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CFocusModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CFocusMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CFocusMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CFocusModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CFocusMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CFocusModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset __x_ABI_CWindows_CMedia_CDevices_CFocusPreset;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CFocusPreset;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CFocusPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CFocusPreset* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CFocusPresetVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CFocusPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CFocusPreset;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CFocusPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CFocusPreset* This,
        __FIIterator_1_Windows__CMedia__CDevices__CFocusPreset** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CFocusPresetVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CFocusPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CHdrVideoMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CInfraredTorchMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        __FIIterator_1_Windows__CMedia__CDevices__CIsoSpeedPreset** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        __FIIterator_1_Windows__CMedia__CDevices__CManualFocusDistance** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* This,
        __FIIterator_1_Windows__CMedia__CDevices__CRegionOfInterest** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        __FIIterator_1_Windows__CMedia__CDevices__CZoomTransitionMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

typedef struct __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl;

interface __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

typedef struct __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        __FIIterator_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl;

interface __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CFocusMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CFocusModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CFocusModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CFocusModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CFocusMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CFocusPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CFocusPresetVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CFocusPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPresetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        UINT32 index,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties;

typedef struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl;

interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance;

typedef struct __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance* result);

    END_INTERFACE
} __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl;

interface __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance
{
    CONST_VTBL struct __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* sender,
        __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel __x_ABI_CWindows_CDevices_CEnumeration_CPanel;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CMediaCaptureDeviceExclusiveControlReleaseMode __x_ABI_CWindows_CMedia_CCapture_CMediaCaptureDeviceExclusiveControlReleaseMode;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CPowerlineFrequency __x_ABI_CWindows_CMedia_CCapture_CPowerlineFrequency;

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat;

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CCameraOcclusionKind __x_ABI_CWindows_CMedia_CDevices_CCameraOcclusionKind;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CCaptureUse __x_ABI_CWindows_CMedia_CDevices_CCaptureUse;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CColorTemperaturePreset __x_ABI_CWindows_CMedia_CDevices_CColorTemperaturePreset;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureFocusState __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureFocusState;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureOptimization __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureOptimization;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CRegionOfInterestType __x_ABI_CWindows_CMedia_CDevices_CRegionOfInterestType;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CSendCommandStatus __x_ABI_CWindows_CMedia_CDevices_CSendCommandStatus;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CTelephonyKey __x_ABI_CWindows_CMedia_CDevices_CTelephonyKey;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerGetDevicePropertyStatus __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerGetDevicePropertyStatus;

typedef enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerSetDevicePropertyStatus __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerSetDevicePropertyStatus;

/*
 *
 * Struct Windows.Media.Devices.AdvancedPhotoMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode
{
    AdvancedPhotoMode_Auto = 0,
    AdvancedPhotoMode_Standard = 1,
    AdvancedPhotoMode_Hdr = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AdvancedPhotoMode_LowLight = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.AudioDeviceRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole
{
    AudioDeviceRole_Default = 0,
    AudioDeviceRole_Communications = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.AutoFocusRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange
{
    AutoFocusRange_FullRange = 0,
    AutoFocusRange_Macro = 1,
    AutoFocusRange_Normal = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CameraOcclusionKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CMedia_CDevices_CCameraOcclusionKind
{
    CameraOcclusionKind_Lid = 0,
    CameraOcclusionKind_CameraHardware = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Media.Devices.CameraStreamState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CCameraStreamState
{
    CameraStreamState_NotStreaming = 0,
    CameraStreamState_Streaming = 1,
    CameraStreamState_BlockedForPrivacy = 2,
    CameraStreamState_Shutdown = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CaptureSceneMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode
{
    CaptureSceneMode_Auto = 0,
    CaptureSceneMode_Manual = 1,
    CaptureSceneMode_Macro = 2,
    CaptureSceneMode_Portrait = 3,
    CaptureSceneMode_Sport = 4,
    CaptureSceneMode_Snow = 5,
    CaptureSceneMode_Night = 6,
    CaptureSceneMode_Beach = 7,
    CaptureSceneMode_Sunset = 8,
    CaptureSceneMode_Candlelight = 9,
    CaptureSceneMode_Landscape = 10,
    CaptureSceneMode_NightPortrait = 11,
    CaptureSceneMode_Backlit = 12,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.CaptureUse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CCaptureUse
{
    CaptureUse_None = 0,
    CaptureUse_Photo = 1,
    CaptureUse_Video = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.ColorTemperaturePreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CColorTemperaturePreset
{
    ColorTemperaturePreset_Auto = 0,
    ColorTemperaturePreset_Manual = 1,
    ColorTemperaturePreset_Cloudy = 2,
    ColorTemperaturePreset_Daylight = 3,
    ColorTemperaturePreset_Flash = 4,
    ColorTemperaturePreset_Fluorescent = 5,
    ColorTemperaturePreset_Tungsten = 6,
    ColorTemperaturePreset_Candlelight = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.DigitalWindowMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode
{
    DigitalWindowMode_Off = 0,
    DigitalWindowMode_On = 1,
    DigitalWindowMode_Auto = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Struct Windows.Media.Devices.FocusMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode
{
    FocusMode_Auto = 0,
    FocusMode_Single = 1,
    FocusMode_Continuous = 2,
    FocusMode_Manual = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.FocusPreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset
{
    FocusPreset_Auto = 0,
    FocusPreset_Manual = 1,
    FocusPreset_AutoMacro = 2,
    FocusPreset_AutoNormal = 3,
    FocusPreset_AutoInfinity = 4,
    FocusPreset_AutoHyperfocal = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.HdrVideoMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode
{
    HdrVideoMode_Off = 0,
    HdrVideoMode_On = 1,
    HdrVideoMode_Auto = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.InfraredTorchMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode
{
    InfraredTorchMode_Off = 0,
    InfraredTorchMode_On = 1,
    InfraredTorchMode_AlternatingFrameIllumination = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Devices.IsoSpeedPreset
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("IsoSpeedPreset may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset
{
    IsoSpeedPreset_Auto = 0,
    IsoSpeedPreset_Iso50 = 1,
    IsoSpeedPreset_Iso80 = 2,
    IsoSpeedPreset_Iso100 = 3,
    IsoSpeedPreset_Iso200 = 4,
    IsoSpeedPreset_Iso400 = 5,
    IsoSpeedPreset_Iso800 = 6,
    IsoSpeedPreset_Iso1600 = 7,
    IsoSpeedPreset_Iso3200 = 8,
    IsoSpeedPreset_Iso6400 = 9,
    IsoSpeedPreset_Iso12800 = 10,
    IsoSpeedPreset_Iso25600 = 11,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.ManualFocusDistance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CManualFocusDistance
{
    ManualFocusDistance_Infinity = 0,
    ManualFocusDistance_Hyperfocal = 1,
    ManualFocusDistance_Nearest = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCaptureFocusState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureFocusState
{
    MediaCaptureFocusState_Uninitialized = 0,
    MediaCaptureFocusState_Lost = 1,
    MediaCaptureFocusState_Searching = 2,
    MediaCaptureFocusState_Focused = 3,
    MediaCaptureFocusState_Failed = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCaptureOptimization
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureOptimization
{
    MediaCaptureOptimization_Default = 0,
    MediaCaptureOptimization_Quality = 1,
    MediaCaptureOptimization_Latency = 2,
    MediaCaptureOptimization_Power = 3,
    MediaCaptureOptimization_LatencyThenQuality = 4,
    MediaCaptureOptimization_LatencyThenPower = 5,
    MediaCaptureOptimization_PowerAndQuality = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.MediaCapturePauseBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CMediaCapturePauseBehavior
{
    MediaCapturePauseBehavior_RetainHardwareResources = 0,
    MediaCapturePauseBehavior_ReleaseHardwareResources = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.OpticalImageStabilizationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode
{
    OpticalImageStabilizationMode_Off = 0,
    OpticalImageStabilizationMode_On = 1,
    OpticalImageStabilizationMode_Auto = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.RegionOfInterestType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CRegionOfInterestType
{
    RegionOfInterestType_Unknown = 0,
    RegionOfInterestType_Face = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.SendCommandStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CDevices_CSendCommandStatus
{
    SendCommandStatus_Success = 0,
    SendCommandStatus_DeviceNotAvailable = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.TelephonyKey
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CTelephonyKey
{
    TelephonyKey_D0 = 0,
    TelephonyKey_D1 = 1,
    TelephonyKey_D2 = 2,
    TelephonyKey_D3 = 3,
    TelephonyKey_D4 = 4,
    TelephonyKey_D5 = 5,
    TelephonyKey_D6 = 6,
    TelephonyKey_D7 = 7,
    TelephonyKey_D8 = 8,
    TelephonyKey_D9 = 9,
    TelephonyKey_Star = 10,
    TelephonyKey_Pound = 11,
    TelephonyKey_A = 12,
    TelephonyKey_B = 13,
    TelephonyKey_C = 14,
    TelephonyKey_D = 15,
};
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerGetDevicePropertyStatus
{
    VideoDeviceControllerGetDevicePropertyStatus_Success = 0,
    VideoDeviceControllerGetDevicePropertyStatus_UnknownFailure = 1,
    VideoDeviceControllerGetDevicePropertyStatus_BufferTooSmall = 2,
    VideoDeviceControllerGetDevicePropertyStatus_NotSupported = 3,
    VideoDeviceControllerGetDevicePropertyStatus_DeviceNotAvailable = 4,
    VideoDeviceControllerGetDevicePropertyStatus_MaxPropertyValueSizeTooSmall = 5,
    VideoDeviceControllerGetDevicePropertyStatus_MaxPropertyValueSizeRequired = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerSetDevicePropertyStatus
{
    VideoDeviceControllerSetDevicePropertyStatus_Success = 0,
    VideoDeviceControllerSetDevicePropertyStatus_UnknownFailure = 1,
    VideoDeviceControllerSetDevicePropertyStatus_NotSupported = 2,
    VideoDeviceControllerSetDevicePropertyStatus_InvalidValue = 3,
    VideoDeviceControllerSetDevicePropertyStatus_DeviceNotAvailable = 4,
    VideoDeviceControllerSetDevicePropertyStatus_NotInControl = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Devices.VideoTemporalDenoisingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode
{
    VideoTemporalDenoisingMode_Off = 0,
    VideoTemporalDenoisingMode_On = 1,
    VideoTemporalDenoisingMode_Auto = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.Devices.ZoomTransitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode
{
    ZoomTransitionMode_Auto = 0,
    ZoomTransitionMode_Direct = 1,
    ZoomTransitionMode_Smooth = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.CallControlEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl* sender);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.DialRequestedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.KeypadPressedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Media.Devices.RedialRequestedEventHandler
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl* sender,
        __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedPhotoCaptureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AdvancedPhotoCaptureSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedPhotoCaptureSettings[] = L"Windows.Media.Devices.IAdvancedPhotoCaptureSettings";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AdvancedPhotoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedPhotoControl[] = L"Windows.Media.Devices.IAdvancedPhotoControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CAdvancedPhotoMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAdvancedPhotoMode* value);
    HRESULT (STDMETHODCALLTYPE* Configure)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoCaptureSettings* settings);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_Configure(This, settings) \
    ((This)->lpVtbl->Configure(This, settings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetDeviceProperty)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        HSTRING propertyId,
        IInspectable* propertyValue);
    HRESULT (STDMETHODCALLTYPE* GetDeviceProperty)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController* This,
        HSTRING propertyId,
        IInspectable** propertyValue);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_SetDeviceProperty(This, propertyId, propertyValue) \
    ((This)->lpVtbl->SetDeviceProperty(This, propertyId, propertyValue))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_GetDeviceProperty(This, propertyId, propertyValue) \
    ((This)->lpVtbl->GetDeviceProperty(This, propertyId, propertyValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController10[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CameraOcclusionInfo)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10* This,
        __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_get_CameraOcclusionInfo(This, value) \
    ((This)->lpVtbl->get_CameraOcclusionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController10_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController11[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryAcquireExclusiveControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCaptureDeviceExclusiveControlReleaseMode mode,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_TryAcquireExclusiveControl(This, deviceId, mode, result) \
    ((This)->lpVtbl->TryAcquireExclusiveControl(This, deviceId, mode, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController11_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController2[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LowLagPhotoSequence)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_LowLagPhoto)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl** value);
    HRESULT (STDMETHODCALLTYPE* get_SceneModeControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl** value);
    HRESULT (STDMETHODCALLTYPE* get_TorchControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CITorchControl** value);
    HRESULT (STDMETHODCALLTYPE* get_FlashControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIFlashControl** value);
    HRESULT (STDMETHODCALLTYPE* get_WhiteBalanceControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_ExposureControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIExposureControl** value);
    HRESULT (STDMETHODCALLTYPE* get_FocusControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIFocusControl** value);
    HRESULT (STDMETHODCALLTYPE* get_ExposureCompensationControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl** value);
    HRESULT (STDMETHODCALLTYPE* get_IsoSpeedControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl** value);
    HRESULT (STDMETHODCALLTYPE* get_RegionsOfInterestControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl** value);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryUse)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureUse* value);
    HRESULT (STDMETHODCALLTYPE* put_PrimaryUse)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureUse value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_LowLagPhotoSequence(This, value) \
    ((This)->lpVtbl->get_LowLagPhotoSequence(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_LowLagPhoto(This, value) \
    ((This)->lpVtbl->get_LowLagPhoto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_SceneModeControl(This, value) \
    ((This)->lpVtbl->get_SceneModeControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_TorchControl(This, value) \
    ((This)->lpVtbl->get_TorchControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_FlashControl(This, value) \
    ((This)->lpVtbl->get_FlashControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_WhiteBalanceControl(This, value) \
    ((This)->lpVtbl->get_WhiteBalanceControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_ExposureControl(This, value) \
    ((This)->lpVtbl->get_ExposureControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_FocusControl(This, value) \
    ((This)->lpVtbl->get_FocusControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_ExposureCompensationControl(This, value) \
    ((This)->lpVtbl->get_ExposureCompensationControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_IsoSpeedControl(This, value) \
    ((This)->lpVtbl->get_IsoSpeedControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_RegionsOfInterestControl(This, value) \
    ((This)->lpVtbl->get_RegionsOfInterestControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_get_PrimaryUse(This, value) \
    ((This)->lpVtbl->get_PrimaryUse(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_put_PrimaryUse(This, value) \
    ((This)->lpVtbl->put_PrimaryUse(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController3[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VariablePhotoSequenceController)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotoConfirmationControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl** value);
    HRESULT (STDMETHODCALLTYPE* get_ZoomControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3* This,
        __x_ABI_CWindows_CMedia_CDevices_CIZoomControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_get_VariablePhotoSequenceController(This, value) \
    ((This)->lpVtbl->get_VariablePhotoSequenceController(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_get_PhotoConfirmationControl(This, value) \
    ((This)->lpVtbl->get_PhotoConfirmationControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_get_ZoomControl(This, value) \
    ((This)->lpVtbl->get_ZoomControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController4[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExposurePriorityVideoControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl** value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredOptimization)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureOptimization* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredOptimization)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureOptimization value);
    HRESULT (STDMETHODCALLTYPE* get_HdrVideoControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl** value);
    HRESULT (STDMETHODCALLTYPE* get_OpticalImageStabilizationControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl** value);
    HRESULT (STDMETHODCALLTYPE* get_AdvancedPhotoControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAdvancedPhotoControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_get_ExposurePriorityVideoControl(This, value) \
    ((This)->lpVtbl->get_ExposurePriorityVideoControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_get_DesiredOptimization(This, value) \
    ((This)->lpVtbl->get_DesiredOptimization(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_put_DesiredOptimization(This, value) \
    ((This)->lpVtbl->put_DesiredOptimization(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_get_HdrVideoControl(This, value) \
    ((This)->lpVtbl->get_HdrVideoControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_get_OpticalImageStabilizationControl(This, value) \
    ((This)->lpVtbl->get_OpticalImageStabilizationControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_get_AdvancedPhotoControl(This, value) \
    ((This)->lpVtbl->get_AdvancedPhotoControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController5[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDevicePropertyById)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        HSTRING propertyId,
        __FIReference_1_UINT32* maxPropertyValueSize,
        __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult** value);
    HRESULT (STDMETHODCALLTYPE* SetDevicePropertyById)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        HSTRING propertyId,
        IInspectable* propertyValue,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerSetDevicePropertyStatus* value);
    HRESULT (STDMETHODCALLTYPE* GetDevicePropertyByExtendedId)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        UINT32 extendedPropertyIdLength,
        BYTE* extendedPropertyId,
        __FIReference_1_UINT32* maxPropertyValueSize,
        __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult** value);
    HRESULT (STDMETHODCALLTYPE* SetDevicePropertyByExtendedId)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5* This,
        UINT32 extendedPropertyIdLength,
        BYTE* extendedPropertyId,
        UINT32 propertyValueLength,
        BYTE* propertyValue,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerSetDevicePropertyStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_GetDevicePropertyById(This, propertyId, maxPropertyValueSize, value) \
    ((This)->lpVtbl->GetDevicePropertyById(This, propertyId, maxPropertyValueSize, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_SetDevicePropertyById(This, propertyId, propertyValue, value) \
    ((This)->lpVtbl->SetDevicePropertyById(This, propertyId, propertyValue, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_GetDevicePropertyByExtendedId(This, extendedPropertyIdLength, extendedPropertyId, maxPropertyValueSize, value) \
    ((This)->lpVtbl->GetDevicePropertyByExtendedId(This, extendedPropertyIdLength, extendedPropertyId, maxPropertyValueSize, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_SetDevicePropertyByExtendedId(This, extendedPropertyIdLength, extendedPropertyId, propertyValueLength, propertyValue, value) \
    ((This)->lpVtbl->SetDevicePropertyByExtendedId(This, extendedPropertyIdLength, extendedPropertyId, propertyValueLength, propertyValue, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController6[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VideoTemporalDenoisingControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6* This,
        __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_get_VideoTemporalDenoisingControl(This, value) \
    ((This)->lpVtbl->get_VideoTemporalDenoisingControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController7[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InfraredTorchControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7* This,
        __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_get_InfraredTorchControl(This, value) \
    ((This)->lpVtbl->get_InfraredTorchControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController8[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PanelBasedOptimizationControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8* This,
        __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_get_PanelBasedOptimizationControl(This, value) \
    ((This)->lpVtbl->get_PanelBasedOptimizationControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAdvancedVideoCaptureDeviceController9[] = L"Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DigitalWindowControl)(__x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9* This,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_get_DigitalWindowControl(This, value) \
    ((This)->lpVtbl->get_DigitalWindowControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAdvancedVideoCaptureDeviceController9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Devices.IMediaDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceController[] = L"Windows.Media.Devices.IAudioDeviceController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Muted)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Muted)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_VolumePercent)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_VolumePercent)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_put_Muted(This, value) \
    ((This)->lpVtbl->put_Muted(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_get_Muted(This, value) \
    ((This)->lpVtbl->get_Muted(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_put_VolumePercent(This, value) \
    ((This)->lpVtbl->put_VolumePercent(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_get_VolumePercent(This, value) \
    ((This)->lpVtbl->get_VolumePercent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceController2[] = L"Windows.Media.Devices.IAudioDeviceController2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioCaptureEffectsManager)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_get_AudioCaptureEffectsManager(This, value) \
    ((This)->lpVtbl->get_AudioCaptureEffectsManager(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModule[] = L"Windows.Media.Devices.IAudioDeviceModule";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClassId)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InstanceId)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MajorVersion)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinorVersion)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SendCommandAsync)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* Command,
        __FIAsyncOperation_1_Windows__CMedia__CDevices__CModuleCommandResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_get_ClassId(This, value) \
    ((This)->lpVtbl->get_ClassId(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_get_InstanceId(This, value) \
    ((This)->lpVtbl->get_InstanceId(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_get_MajorVersion(This, value) \
    ((This)->lpVtbl->get_MajorVersion(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_get_MinorVersion(This, value) \
    ((This)->lpVtbl->get_MinorVersion(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_SendCommandAsync(This, Command, operation) \
    ((This)->lpVtbl->SendCommandAsync(This, Command, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModuleNotificationEventArgs[] = L"Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Module)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModule** value);
    HRESULT (STDMETHODCALLTYPE* get_NotificationData)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_get_Module(This, value) \
    ((This)->lpVtbl->get_Module(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_get_NotificationData(This, value) \
    ((This)->lpVtbl->get_NotificationData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModuleNotificationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModulesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModulesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModulesManager[] = L"Windows.Media.Devices.IAudioDeviceModulesManager";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ModuleNotificationReceived)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        __FITypedEventHandler_2_Windows__CMedia__CDevices__CAudioDeviceModulesManager_Windows__CMedia__CDevices__CAudioDeviceModuleNotificationEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ModuleNotificationReceived)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* FindAllById)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        HSTRING moduleId,
        __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule** modules);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CAudioDeviceModule** modules);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_add_ModuleNotificationReceived(This, handler, token) \
    ((This)->lpVtbl->add_ModuleNotificationReceived(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_remove_ModuleNotificationReceived(This, token) \
    ((This)->lpVtbl->remove_ModuleNotificationReceived(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FindAllById(This, moduleId, modules) \
    ((This)->lpVtbl->FindAllById(This, moduleId, modules))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_FindAll(This, modules) \
    ((This)->lpVtbl->FindAll(This, modules))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IAudioDeviceModulesManagerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.AudioDeviceModulesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IAudioDeviceModulesManagerFactory[] = L"Windows.Media.Devices.IAudioDeviceModulesManagerFactory";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_Create(This, deviceId, result) \
    ((This)->lpVtbl->Create(This, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceModulesManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.ICallControl
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CallControl
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICallControl[] = L"Windows.Media.Devices.ICallControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICallControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IndicateNewIncomingCall)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        boolean enableRinger,
        HSTRING callerId,
        UINT64* callToken);
    HRESULT (STDMETHODCALLTYPE* IndicateNewOutgoingCall)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        UINT64* callToken);
    HRESULT (STDMETHODCALLTYPE* IndicateActiveCall)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        UINT64 callToken);
    HRESULT (STDMETHODCALLTYPE* EndCall)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        UINT64 callToken);
    HRESULT (STDMETHODCALLTYPE* get_HasRinger)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_AnswerRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AnswerRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HangUpRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HangUpRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DialRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DialRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RedialRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RedialRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_KeypadPressed)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_KeypadPressed)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AudioTransferRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControlEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioTransferRequested)(__x_ABI_CWindows_CMedia_CDevices_CICallControl* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICallControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICallControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICallControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_IndicateNewIncomingCall(This, enableRinger, callerId, callToken) \
    ((This)->lpVtbl->IndicateNewIncomingCall(This, enableRinger, callerId, callToken))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_IndicateNewOutgoingCall(This, callToken) \
    ((This)->lpVtbl->IndicateNewOutgoingCall(This, callToken))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_IndicateActiveCall(This, callToken) \
    ((This)->lpVtbl->IndicateActiveCall(This, callToken))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_EndCall(This, callToken) \
    ((This)->lpVtbl->EndCall(This, callToken))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_get_HasRinger(This, value) \
    ((This)->lpVtbl->get_HasRinger(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_AnswerRequested(This, handler, token) \
    ((This)->lpVtbl->add_AnswerRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_AnswerRequested(This, token) \
    ((This)->lpVtbl->remove_AnswerRequested(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_HangUpRequested(This, handler, token) \
    ((This)->lpVtbl->add_HangUpRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_HangUpRequested(This, token) \
    ((This)->lpVtbl->remove_HangUpRequested(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_DialRequested(This, handler, token) \
    ((This)->lpVtbl->add_DialRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_DialRequested(This, token) \
    ((This)->lpVtbl->remove_DialRequested(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_RedialRequested(This, handler, token) \
    ((This)->lpVtbl->add_RedialRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_RedialRequested(This, token) \
    ((This)->lpVtbl->remove_RedialRequested(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_KeypadPressed(This, handler, token) \
    ((This)->lpVtbl->add_KeypadPressed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_KeypadPressed(This, token) \
    ((This)->lpVtbl->remove_KeypadPressed(This, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_add_AudioTransferRequested(This, handler, token) \
    ((This)->lpVtbl->add_AudioTransferRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControl_remove_AudioTransferRequested(This, token) \
    ((This)->lpVtbl->remove_AudioTransferRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ICallControlStatics
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CallControl
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICallControlStatics[] = L"Windows.Media.Devices.ICallControlStatics";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICallControlStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl** callControl);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CMedia_CDevices_CICallControlStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CDevices_CICallControl** callControl);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICallControlStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICallControlStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_GetDefault(This, callControl) \
    ((This)->lpVtbl->GetDefault(This, callControl))

#define __x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_FromId(This, deviceId, callControl) \
    ((This)->lpVtbl->FromId(This, deviceId, callControl))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICallControlStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICallControlStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionInfo[] = L"Windows.Media.Devices.ICameraOcclusionInfo";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetState)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState** result);
    HRESULT (STDMETHODCALLTYPE* IsOcclusionKindSupported)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCameraOcclusionKind occlusionKind,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        __FITypedEventHandler_2_Windows__CMedia__CDevices__CCameraOcclusionInfo_Windows__CMedia__CDevices__CCameraOcclusionStateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfoVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_GetState(This, result) \
    ((This)->lpVtbl->GetState(This, result))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_IsOcclusionKindSupported(This, occlusionKind, result) \
    ((This)->lpVtbl->IsOcclusionKindSupported(This, occlusionKind, result))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionState[] = L"Windows.Media.Devices.ICameraOcclusionState";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsOccluded)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsOcclusionKind)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCameraOcclusionKind occlusionKind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_get_IsOccluded(This, value) \
    ((This)->lpVtbl->get_IsOccluded(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_IsOcclusionKind(This, occlusionKind, result) \
    ((This)->lpVtbl->IsOcclusionKind(This, occlusionKind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.CameraOcclusionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ICameraOcclusionStateChangedEventArgs[] = L"Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionState** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CICameraOcclusionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDefaultAudioDeviceChangedEventArgs[] = L"Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Role)(__x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_get_Role(This, value) \
    ((This)->lpVtbl->get_Role(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDefaultAudioDeviceChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IDialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DialRequestedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDialRequestedEventArgs[] = L"Windows.Media.Devices.IDialRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Handled)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_Handled(This) \
    ((This)->lpVtbl->Handled(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDialRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowBounds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowBounds[] = L"Windows.Media.Devices.IDigitalWindowBounds";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBoundsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NormalizedOriginTop)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_NormalizedOriginTop)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_NormalizedOriginLeft)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_NormalizedOriginLeft)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Scale)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Scale)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBoundsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBoundsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_get_NormalizedOriginTop(This, value) \
    ((This)->lpVtbl->get_NormalizedOriginTop(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_put_NormalizedOriginTop(This, value) \
    ((This)->lpVtbl->put_NormalizedOriginTop(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_get_NormalizedOriginLeft(This, value) \
    ((This)->lpVtbl->get_NormalizedOriginLeft(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_put_NormalizedOriginLeft(This, value) \
    ((This)->lpVtbl->put_NormalizedOriginLeft(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_get_Scale(This, value) \
    ((This)->lpVtbl->get_Scale(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_put_Scale(This, value) \
    ((This)->lpVtbl->put_Scale(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowCapability[] = L"Windows.Media.Devices.IDigitalWindowCapability";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinScaleValue)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxScaleValue)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_MinScaleValueWithoutUpsampling)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_NormalizedFieldOfViewLimit)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapabilityVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_MinScaleValue(This, value) \
    ((This)->lpVtbl->get_MinScaleValue(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_MaxScaleValue(This, value) \
    ((This)->lpVtbl->get_MaxScaleValue(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_MinScaleValueWithoutUpsampling(This, value) \
    ((This)->lpVtbl->get_MinScaleValueWithoutUpsampling(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_get_NormalizedFieldOfViewLimit(This, value) \
    ((This)->lpVtbl->get_NormalizedFieldOfViewLimit(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IDigitalWindowControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.DigitalWindowControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IDigitalWindowControl[] = L"Windows.Media.Devices.IDigitalWindowControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        UINT32* valueLength,
        enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentMode)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode* value);
    HRESULT (STDMETHODCALLTYPE* GetBounds)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds** result);
    HRESULT (STDMETHODCALLTYPE* Configure)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode digitalWindowMode);
    HRESULT (STDMETHODCALLTYPE* ConfigureWithBounds)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CDigitalWindowMode digitalWindowMode,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowBounds* digitalWindowBounds);
    HRESULT (STDMETHODCALLTYPE* get_SupportedCapabilities)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CDigitalWindowCapability** value);
    HRESULT (STDMETHODCALLTYPE* GetCapabilityForSize)(__x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl* This,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowCapability** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_get_SupportedModes(This, valueLength, value) \
    ((This)->lpVtbl->get_SupportedModes(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_get_CurrentMode(This, value) \
    ((This)->lpVtbl->get_CurrentMode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_GetBounds(This, result) \
    ((This)->lpVtbl->GetBounds(This, result))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_Configure(This, digitalWindowMode) \
    ((This)->lpVtbl->Configure(This, digitalWindowMode))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_ConfigureWithBounds(This, digitalWindowMode, digitalWindowBounds) \
    ((This)->lpVtbl->ConfigureWithBounds(This, digitalWindowMode, digitalWindowBounds))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_get_SupportedCapabilities(This, value) \
    ((This)->lpVtbl->get_SupportedCapabilities(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_GetCapabilityForSize(This, width, height, result) \
    ((This)->lpVtbl->GetCapabilityForSize(This, width, height, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIDigitalWindowControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Media.Devices.IExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposureCompensationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposureCompensationControl[] = L"Windows.Media.Devices.IExposureCompensationControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl* This,
        FLOAT value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_SetValueAsync(This, value, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, value, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureCompensationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposureControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposureControl[] = L"Windows.Media.Devices.IExposureControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIExposureControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetAutoAsync)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        boolean value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CIExposureControl* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan shutterDuration,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIExposureControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIExposureControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIExposureControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_SetAutoAsync(This, value, asyncInfo) \
    ((This)->lpVtbl->SetAutoAsync(This, value, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposureControl_SetValueAsync(This, shutterDuration, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, shutterDuration, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposureControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposureControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IExposurePriorityVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ExposurePriorityVideoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IExposurePriorityVideoControl[] = L"Windows.Media.Devices.IExposurePriorityVideoControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIExposurePriorityVideoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFlashControl[] = L"Windows.Media.Devices.IFlashControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIFlashControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PowerSupported)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RedEyeReductionSupported)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Auto)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RedEyeReduction)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RedEyeReduction)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIFlashControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIFlashControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIFlashControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_PowerSupported(This, value) \
    ((This)->lpVtbl->get_PowerSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_RedEyeReductionSupported(This, value) \
    ((This)->lpVtbl->get_RedEyeReductionSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_put_Auto(This, value) \
    ((This)->lpVtbl->put_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_RedEyeReduction(This, value) \
    ((This)->lpVtbl->get_RedEyeReduction(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_put_RedEyeReduction(This, value) \
    ((This)->lpVtbl->put_RedEyeReduction(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_get_PowerPercent(This, value) \
    ((This)->lpVtbl->get_PowerPercent(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl_put_PowerPercent(This, value) \
    ((This)->lpVtbl->put_PowerPercent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFlashControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFlashControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFlashControl2[] = L"Windows.Media.Devices.IFlashControl2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AssistantLightSupported)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AssistantLightEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AssistantLightEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIFlashControl2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_get_AssistantLightSupported(This, value) \
    ((This)->lpVtbl->get_AssistantLightSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_get_AssistantLightEnabled(This, value) \
    ((This)->lpVtbl->get_AssistantLightEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_put_AssistantLightEnabled(This, value) \
    ((This)->lpVtbl->put_AssistantLightEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFlashControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFlashControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusControl[] = L"Windows.Media.Devices.IFocusControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIFocusControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedPresets)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CFocusPreset** value);
    HRESULT (STDMETHODCALLTYPE* get_Preset)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset* value);
    HRESULT (STDMETHODCALLTYPE* SetPresetAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset preset,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetPresetWithCompletionOptionAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusPreset preset,
        boolean completeBeforeFocus,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        UINT32 focus,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FocusAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIFocusControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIFocusControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIFocusControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_SupportedPresets(This, value) \
    ((This)->lpVtbl->get_SupportedPresets(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Preset(This, value) \
    ((This)->lpVtbl->get_Preset(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_SetPresetAsync(This, preset, asyncInfo) \
    ((This)->lpVtbl->SetPresetAsync(This, preset, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_SetPresetWithCompletionOptionAsync(This, preset, completeBeforeFocus, asyncInfo) \
    ((This)->lpVtbl->SetPresetWithCompletionOptionAsync(This, preset, completeBeforeFocus, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_SetValueAsync(This, focus, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, focus, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl_FocusAsync(This, asyncInfo) \
    ((This)->lpVtbl->FocusAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusControl2[] = L"Windows.Media.Devices.IFocusControl2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FocusChangedSupported)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WaitForFocusSupported)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFocusModes)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CFocusMode** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFocusDistances)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CManualFocusDistance** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFocusRanges)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CAutoFocusRange** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* value);
    HRESULT (STDMETHODCALLTYPE* get_FocusState)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CMediaCaptureFocusState* value);
    HRESULT (STDMETHODCALLTYPE* UnlockAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LockAsync)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* Configure)(__x_ABI_CWindows_CMedia_CDevices_CIFocusControl2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* settings);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_FocusChangedSupported(This, value) \
    ((This)->lpVtbl->get_FocusChangedSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_WaitForFocusSupported(This, value) \
    ((This)->lpVtbl->get_WaitForFocusSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_SupportedFocusModes(This, value) \
    ((This)->lpVtbl->get_SupportedFocusModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_SupportedFocusDistances(This, value) \
    ((This)->lpVtbl->get_SupportedFocusDistances(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_SupportedFocusRanges(This, value) \
    ((This)->lpVtbl->get_SupportedFocusRanges(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_get_FocusState(This, value) \
    ((This)->lpVtbl->get_FocusState(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_UnlockAsync(This, asyncInfo) \
    ((This)->lpVtbl->UnlockAsync(This, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_LockAsync(This, asyncInfo) \
    ((This)->lpVtbl->LockAsync(This, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_Configure(This, settings) \
    ((This)->lpVtbl->Configure(This, settings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IFocusSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.FocusSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IFocusSettings[] = L"Windows.Media.Devices.IFocusSettings";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIFocusSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CFocusMode value);
    HRESULT (STDMETHODCALLTYPE* get_AutoFocusRange)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoFocusRange)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAutoFocusRange value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        __FIReference_1_UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Distance)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance** value);
    HRESULT (STDMETHODCALLTYPE* put_Distance)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        __FIReference_1_Windows__CMedia__CDevices__CManualFocusDistance* value);
    HRESULT (STDMETHODCALLTYPE* get_WaitForFocus)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_WaitForFocus)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DisableDriverFallback)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DisableDriverFallback)(__x_ABI_CWindows_CMedia_CDevices_CIFocusSettings* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIFocusSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIFocusSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_AutoFocusRange(This, value) \
    ((This)->lpVtbl->get_AutoFocusRange(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_AutoFocusRange(This, value) \
    ((This)->lpVtbl->put_AutoFocusRange(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_Distance(This, value) \
    ((This)->lpVtbl->get_Distance(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_Distance(This, value) \
    ((This)->lpVtbl->put_Distance(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_WaitForFocus(This, value) \
    ((This)->lpVtbl->get_WaitForFocus(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_WaitForFocus(This, value) \
    ((This)->lpVtbl->put_WaitForFocus(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_get_DisableDriverFallback(This, value) \
    ((This)->lpVtbl->get_DisableDriverFallback(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_put_DisableDriverFallback(This, value) \
    ((This)->lpVtbl->put_DisableDriverFallback(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIFocusSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIFocusSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IHdrVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.HdrVideoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IHdrVideoControl[] = L"Windows.Media.Devices.IHdrVideoControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CHdrVideoMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CHdrVideoMode value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIHdrVideoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IInfraredTorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.InfraredTorchControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IInfraredTorchControl[] = L"Windows.Media.Devices.IInfraredTorchControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CInfraredTorchMode** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentMode)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode* value);
    HRESULT (STDMETHODCALLTYPE* put_CurrentMode)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CInfraredTorchMode value);
    HRESULT (STDMETHODCALLTYPE* get_MinPower)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPower)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PowerStep)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Power)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Power)(__x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_CurrentMode(This, value) \
    ((This)->lpVtbl->get_CurrentMode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_put_CurrentMode(This, value) \
    ((This)->lpVtbl->put_CurrentMode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_MinPower(This, value) \
    ((This)->lpVtbl->get_MinPower(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_MaxPower(This, value) \
    ((This)->lpVtbl->get_MaxPower(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_PowerStep(This, value) \
    ((This)->lpVtbl->get_PowerStep(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_get_Power(This, value) \
    ((This)->lpVtbl->get_Power(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_put_Power(This, value) \
    ((This)->lpVtbl->put_Power(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIInfraredTorchControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Devices.IIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.IsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IIsoSpeedControl[] = L"Windows.Media.Devices.IIsoSpeedControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SupportedPresets may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SupportedPresets)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CIsoSpeedPreset** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Preset may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Preset)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SetPresetAsync may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetPresetAsync)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CIsoSpeedPreset preset,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SupportedPresets may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_get_SupportedPresets(This, value) \
    ((This)->lpVtbl->get_SupportedPresets(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Preset may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_get_Preset(This, value) \
    ((This)->lpVtbl->get_Preset(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SetPresetAsync may not be available in future versions of Windows Phone. Starting with Windows Phone 8.1, use SetAutoAsync, Auto, SetValueAsync, and Value instead")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_SetPresetAsync(This, preset, asyncInfo) \
    ((This)->lpVtbl->SetPresetAsync(This, preset, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IIsoSpeedControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.IsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IIsoSpeedControl2[] = L"Windows.Media.Devices.IIsoSpeedControl2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        UINT32 isoSpeed,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetAutoAsync)(__x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_SetValueAsync(This, isoSpeed, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, isoSpeed, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_SetAutoAsync(This, asyncInfo) \
    ((This)->lpVtbl->SetAutoAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIIsoSpeedControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IKeypadPressedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.KeypadPressedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IKeypadPressedEventArgs[] = L"Windows.Media.Devices.IKeypadPressedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TelephonyKey)(__x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CTelephonyKey* telephonyKey);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_get_TelephonyKey(This, telephonyKey) \
    ((This)->lpVtbl->get_TelephonyKey(This, telephonyKey))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIKeypadPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ILowLagPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.LowLagPhotoControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ILowLagPhotoControl[] = L"Windows.Media.Devices.ILowLagPhotoControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetHighestConcurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* captureProperties,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbnailEnabled)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ThumbnailEnabled)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbnailFormat)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_ThumbnailFormat)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredThumbnailSize)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredThumbnailSize)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareAcceleratedThumbnailSupported)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_GetHighestConcurrentFrameRate(This, captureProperties, value) \
    ((This)->lpVtbl->GetHighestConcurrentFrameRate(This, captureProperties, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_GetCurrentFrameRate(This, value) \
    ((This)->lpVtbl->GetCurrentFrameRate(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_get_ThumbnailEnabled(This, value) \
    ((This)->lpVtbl->get_ThumbnailEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_put_ThumbnailEnabled(This, value) \
    ((This)->lpVtbl->put_ThumbnailEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_get_ThumbnailFormat(This, value) \
    ((This)->lpVtbl->get_ThumbnailFormat(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_put_ThumbnailFormat(This, value) \
    ((This)->lpVtbl->put_ThumbnailFormat(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_get_DesiredThumbnailSize(This, value) \
    ((This)->lpVtbl->get_DesiredThumbnailSize(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_put_DesiredThumbnailSize(This, value) \
    ((This)->lpVtbl->put_DesiredThumbnailSize(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_get_HardwareAcceleratedThumbnailSupported(This, value) \
    ((This)->lpVtbl->get_HardwareAcceleratedThumbnailSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ILowLagPhotoSequenceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.LowLagPhotoSequenceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ILowLagPhotoSequenceControl[] = L"Windows.Media.Devices.ILowLagPhotoSequenceControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPastPhotos)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPhotosPerSecond)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_PastPhotoLimit)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_PastPhotoLimit)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosPerSecondLimit)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PhotosPerSecondLimit)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* GetHighestConcurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* captureProperties,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbnailEnabled)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ThumbnailEnabled)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ThumbnailFormat)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_ThumbnailFormat)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredThumbnailSize)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredThumbnailSize)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareAcceleratedThumbnailSupported)(__x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_MaxPastPhotos(This, value) \
    ((This)->lpVtbl->get_MaxPastPhotos(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_MaxPhotosPerSecond(This, value) \
    ((This)->lpVtbl->get_MaxPhotosPerSecond(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_PastPhotoLimit(This, value) \
    ((This)->lpVtbl->get_PastPhotoLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_put_PastPhotoLimit(This, value) \
    ((This)->lpVtbl->put_PastPhotoLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_PhotosPerSecondLimit(This, value) \
    ((This)->lpVtbl->get_PhotosPerSecondLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_put_PhotosPerSecondLimit(This, value) \
    ((This)->lpVtbl->put_PhotosPerSecondLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_GetHighestConcurrentFrameRate(This, captureProperties, value) \
    ((This)->lpVtbl->GetHighestConcurrentFrameRate(This, captureProperties, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_GetCurrentFrameRate(This, value) \
    ((This)->lpVtbl->GetCurrentFrameRate(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_ThumbnailEnabled(This, value) \
    ((This)->lpVtbl->get_ThumbnailEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_put_ThumbnailEnabled(This, value) \
    ((This)->lpVtbl->put_ThumbnailEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_ThumbnailFormat(This, value) \
    ((This)->lpVtbl->get_ThumbnailFormat(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_put_ThumbnailFormat(This, value) \
    ((This)->lpVtbl->put_ThumbnailFormat(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_DesiredThumbnailSize(This, value) \
    ((This)->lpVtbl->get_DesiredThumbnailSize(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_put_DesiredThumbnailSize(This, value) \
    ((This)->lpVtbl->put_DesiredThumbnailSize(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_get_HardwareAcceleratedThumbnailSupported(This, value) \
    ((This)->lpVtbl->get_HardwareAcceleratedThumbnailSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CILowLagPhotoSequenceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDeviceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceControl[] = L"Windows.Media.Devices.IMediaDeviceControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capabilities)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* TryGetValue)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        DOUBLE* value,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* TrySetValue)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        DOUBLE value,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* TryGetAuto)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        boolean* value,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* TrySetAuto)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl* This,
        boolean value,
        boolean* succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_get_Capabilities(This, value) \
    ((This)->lpVtbl->get_Capabilities(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_TryGetValue(This, value, succeeded) \
    ((This)->lpVtbl->TryGetValue(This, value, succeeded))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_TrySetValue(This, value, succeeded) \
    ((This)->lpVtbl->TrySetValue(This, value, succeeded))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_TryGetAuto(This, value, succeeded) \
    ((This)->lpVtbl->TryGetAuto(This, value, succeeded))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_TrySetAuto(This, value, succeeded) \
    ((This)->lpVtbl->TrySetAuto(This, value, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDeviceControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceControlCapabilities[] = L"Windows.Media.Devices.IMediaDeviceControlCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Default)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoModeSupported)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_Default(This, value) \
    ((This)->lpVtbl->get_Default(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_get_AutoModeSupported(This, value) \
    ((This)->lpVtbl->get_AutoModeSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControlCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceController[] = L"Windows.Media.Devices.IMediaDeviceController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAvailableMediaStreamProperties)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType mediaStreamType,
        __FIVectorView_1_Windows__CMedia__CMediaProperties__CIMediaEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* GetMediaStreamProperties)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType mediaStreamType,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* SetMediaStreamPropertiesAsync)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType mediaStreamType,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* mediaEncodingProperties,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_GetAvailableMediaStreamProperties(This, mediaStreamType, value) \
    ((This)->lpVtbl->GetAvailableMediaStreamProperties(This, mediaStreamType, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_GetMediaStreamProperties(This, mediaStreamType, value) \
    ((This)->lpVtbl->GetMediaStreamProperties(This, mediaStreamType, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_SetMediaStreamPropertiesAsync(This, mediaStreamType, mediaEncodingProperties, asyncInfo) \
    ((This)->lpVtbl->SetMediaStreamPropertiesAsync(This, mediaStreamType, mediaEncodingProperties, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IMediaDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.MediaDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IMediaDeviceStatics[] = L"Windows.Media.Devices.IMediaDeviceStatics";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAudioCaptureSelector)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetAudioRenderSelector)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetVideoCaptureSelector)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAudioCaptureId)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole role,
        HSTRING* deviceId);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAudioRenderId)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole role,
        HSTRING* deviceId);
    HRESULT (STDMETHODCALLTYPE* add_DefaultAudioCaptureDeviceChanged)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioCaptureDeviceChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_DefaultAudioCaptureDeviceChanged)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_DefaultAudioRenderDeviceChanged)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        __FITypedEventHandler_2_IInspectable_Windows__CMedia__CDevices__CDefaultAudioRenderDeviceChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_DefaultAudioRenderDeviceChanged)(__x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetAudioCaptureSelector(This, selector) \
    ((This)->lpVtbl->GetAudioCaptureSelector(This, selector))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetAudioRenderSelector(This, selector) \
    ((This)->lpVtbl->GetAudioRenderSelector(This, selector))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetVideoCaptureSelector(This, selector) \
    ((This)->lpVtbl->GetVideoCaptureSelector(This, selector))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetDefaultAudioCaptureId(This, role, deviceId) \
    ((This)->lpVtbl->GetDefaultAudioCaptureId(This, role, deviceId))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_GetDefaultAudioRenderId(This, role, deviceId) \
    ((This)->lpVtbl->GetDefaultAudioRenderId(This, role, deviceId))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_add_DefaultAudioCaptureDeviceChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_DefaultAudioCaptureDeviceChanged(This, handler, cookie))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_remove_DefaultAudioCaptureDeviceChanged(This, cookie) \
    ((This)->lpVtbl->remove_DefaultAudioCaptureDeviceChanged(This, cookie))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_add_DefaultAudioRenderDeviceChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_DefaultAudioRenderDeviceChanged(This, handler, cookie))

#define __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_remove_DefaultAudioRenderDeviceChanged(This, cookie) \
    ((This)->lpVtbl->remove_DefaultAudioRenderDeviceChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IModuleCommandResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ModuleCommandResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IModuleCommandResult[] = L"Windows.Media.Devices.IModuleCommandResult";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CSendCommandStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResultVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIModuleCommandResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IOpticalImageStabilizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.OpticalImageStabilizationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IOpticalImageStabilizationControl[] = L"Windows.Media.Devices.IOpticalImageStabilizationControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__COpticalImageStabilizationMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_COpticalImageStabilizationMode value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIOpticalImageStabilizationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IPanelBasedOptimizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.PanelBasedOptimizationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IPanelBasedOptimizationControl[] = L"Windows.Media.Devices.IPanelBasedOptimizationControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Panel)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel* value);
    HRESULT (STDMETHODCALLTYPE* put_Panel)(__x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_get_Panel(This, value) \
    ((This)->lpVtbl->get_Panel(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_put_Panel(This, value) \
    ((This)->lpVtbl->put_Panel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIPanelBasedOptimizationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Devices.IPhotoConfirmationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.PhotoConfirmationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IPhotoConfirmationControl[] = L"Windows.Media.Devices.IPhotoConfirmationControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        boolean* pbSupported);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PixelFormat)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat* format);
    HRESULT (STDMETHODCALLTYPE* put_PixelFormat)(__x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat format);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_get_Supported(This, pbSupported) \
    ((This)->lpVtbl->get_Supported(This, pbSupported))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_get_PixelFormat(This, format) \
    ((This)->lpVtbl->get_PixelFormat(This, format))

#define __x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_put_PixelFormat(This, format) \
    ((This)->lpVtbl->put_PixelFormat(This, format))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIPhotoConfirmationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRedialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RedialRequestedEventArgs
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRedialRequestedEventArgs[] = L"Windows.Media.Devices.IRedialRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Handled)(__x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_Handled(This) \
    ((This)->lpVtbl->Handled(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRedialRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionOfInterest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionOfInterest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionOfInterest[] = L"Windows.Media.Devices.IRegionOfInterest";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutoFocusEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoFocusEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AutoWhiteBalanceEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoWhiteBalanceEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AutoExposureEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoExposureEnabled)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Bounds)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_Bounds)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterestVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_get_AutoFocusEnabled(This, value) \
    ((This)->lpVtbl->get_AutoFocusEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_put_AutoFocusEnabled(This, value) \
    ((This)->lpVtbl->put_AutoFocusEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_get_AutoWhiteBalanceEnabled(This, value) \
    ((This)->lpVtbl->get_AutoWhiteBalanceEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_put_AutoWhiteBalanceEnabled(This, value) \
    ((This)->lpVtbl->put_AutoWhiteBalanceEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_get_AutoExposureEnabled(This, value) \
    ((This)->lpVtbl->get_AutoExposureEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_put_AutoExposureEnabled(This, value) \
    ((This)->lpVtbl->put_AutoExposureEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_get_Bounds(This, value) \
    ((This)->lpVtbl->get_Bounds(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_put_Bounds(This, value) \
    ((This)->lpVtbl->put_Bounds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionOfInterest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionOfInterest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionOfInterest2[] = L"Windows.Media.Devices.IRegionOfInterest2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CRegionOfInterestType* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CRegionOfInterestType value);
    HRESULT (STDMETHODCALLTYPE* get_BoundsNormalized)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_BoundsNormalized)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Weight)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Weight)(__x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_get_BoundsNormalized(This, value) \
    ((This)->lpVtbl->get_BoundsNormalized(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_put_BoundsNormalized(This, value) \
    ((This)->lpVtbl->put_BoundsNormalized(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_get_Weight(This, value) \
    ((This)->lpVtbl->get_Weight(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_put_Weight(This, value) \
    ((This)->lpVtbl->put_Weight(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionOfInterest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IRegionsOfInterestControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.RegionsOfInterestControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IRegionsOfInterestControl[] = L"Windows.Media.Devices.IRegionsOfInterestControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxRegions)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetRegionsAsync)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* regions,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetRegionsWithLockAsync)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        __FIIterable_1_Windows__CMedia__CDevices__CRegionOfInterest* regions,
        boolean lockValues,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* ClearRegionsAsync)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_AutoFocusSupported)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoWhiteBalanceSupported)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoExposureSupported)(__x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_get_MaxRegions(This, value) \
    ((This)->lpVtbl->get_MaxRegions(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_SetRegionsAsync(This, regions, asyncInfo) \
    ((This)->lpVtbl->SetRegionsAsync(This, regions, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_SetRegionsWithLockAsync(This, regions, lockValues, asyncInfo) \
    ((This)->lpVtbl->SetRegionsWithLockAsync(This, regions, lockValues, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_ClearRegionsAsync(This, asyncInfo) \
    ((This)->lpVtbl->ClearRegionsAsync(This, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_get_AutoFocusSupported(This, value) \
    ((This)->lpVtbl->get_AutoFocusSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_get_AutoWhiteBalanceSupported(This, value) \
    ((This)->lpVtbl->get_AutoWhiteBalanceSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_get_AutoExposureSupported(This, value) \
    ((This)->lpVtbl->get_AutoExposureSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIRegionsOfInterestControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ISceneModeControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.SceneModeControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ISceneModeControl[] = L"Windows.Media.Devices.ISceneModeControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CISceneModeControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CCaptureSceneMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CISceneModeControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCaptureSceneMode sceneMode,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CISceneModeControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CISceneModeControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_SetValueAsync(This, sceneMode, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, sceneMode, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CISceneModeControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CISceneModeControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.ITorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.TorchControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_ITorchControl[] = L"Windows.Media.Devices.ITorchControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CITorchControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PowerSupported)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CITorchControl* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CITorchControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CITorchControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CITorchControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_get_PowerSupported(This, value) \
    ((This)->lpVtbl->get_PowerSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_get_PowerPercent(This, value) \
    ((This)->lpVtbl->get_PowerPercent(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CITorchControl_put_PowerPercent(This, value) \
    ((This)->lpVtbl->put_PowerPercent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CITorchControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CITorchControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IVideoDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceController
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Devices.IMediaDeviceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoDeviceController[] = L"Windows.Media.Devices.IVideoDeviceController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Brightness)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Contrast)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Hue)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_WhiteBalance)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_BacklightCompensation)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Pan)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Tilt)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Zoom)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Roll)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Exposure)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* get_Focus)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIMediaDeviceControl** value);
    HRESULT (STDMETHODCALLTYPE* TrySetPowerlineFrequency)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CPowerlineFrequency value,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* TryGetPowerlineFrequency)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CPowerlineFrequency* value,
        boolean* succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Brightness(This, value) \
    ((This)->lpVtbl->get_Brightness(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Contrast(This, value) \
    ((This)->lpVtbl->get_Contrast(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Hue(This, value) \
    ((This)->lpVtbl->get_Hue(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_WhiteBalance(This, value) \
    ((This)->lpVtbl->get_WhiteBalance(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_BacklightCompensation(This, value) \
    ((This)->lpVtbl->get_BacklightCompensation(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Pan(This, value) \
    ((This)->lpVtbl->get_Pan(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Tilt(This, value) \
    ((This)->lpVtbl->get_Tilt(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Zoom(This, value) \
    ((This)->lpVtbl->get_Zoom(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Roll(This, value) \
    ((This)->lpVtbl->get_Roll(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Exposure(This, value) \
    ((This)->lpVtbl->get_Exposure(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_get_Focus(This, value) \
    ((This)->lpVtbl->get_Focus(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_TrySetPowerlineFrequency(This, value, succeeded) \
    ((This)->lpVtbl->TrySetPowerlineFrequency(This, value, succeeded))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_TryGetPowerlineFrequency(This, value, succeeded) \
    ((This)->lpVtbl->TryGetPowerlineFrequency(This, value, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoDeviceControllerGetDevicePropertyResult[] = L"Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoDeviceControllerGetDevicePropertyStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResultVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceControllerGetDevicePropertyResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Devices.IVideoTemporalDenoisingControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.VideoTemporalDenoisingControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IVideoTemporalDenoisingControl[] = L"Windows.Media.Devices.IVideoTemporalDenoisingControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CVideoTemporalDenoisingMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CVideoTemporalDenoisingMode value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIVideoTemporalDenoisingControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Devices.IWhiteBalanceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.WhiteBalanceControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IWhiteBalanceControl[] = L"Windows.Media.Devices.IWhiteBalanceControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Preset)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CColorTemperaturePreset* value);
    HRESULT (STDMETHODCALLTYPE* SetPresetAsync)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CColorTemperaturePreset preset,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetValueAsync)(__x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl* This,
        UINT32 temperature,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Preset(This, value) \
    ((This)->lpVtbl->get_Preset(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_SetPresetAsync(This, preset, asyncInfo) \
    ((This)->lpVtbl->SetPresetAsync(This, preset, asyncInfo))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_SetValueAsync(This, temperature, asyncInfo) \
    ((This)->lpVtbl->SetValueAsync(This, temperature, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIWhiteBalanceControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomControl[] = L"Windows.Media.Devices.IZoomControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIZoomControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIZoomControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIZoomControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIZoomControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomControl2[] = L"Windows.Media.Devices.IZoomControl2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModes)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CZoomTransitionMode** value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* value);
    HRESULT (STDMETHODCALLTYPE* Configure)(__x_ABI_CWindows_CMedia_CDevices_CIZoomControl2* This,
        __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* settings);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_get_SupportedModes(This, value) \
    ((This)->lpVtbl->get_SupportedModes(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_Configure(This, settings) \
    ((This)->lpVtbl->Configure(This, settings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomControl2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.IZoomSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.ZoomSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_IZoomSettings[] = L"Windows.Media.Devices.IZoomSettings";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CIZoomSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CZoomTransitionMode value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CIZoomSettings* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CIZoomSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CIZoomSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CIZoomSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CIZoomSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AdvancedPhotoCaptureSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAdvancedPhotoCaptureSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoCaptureSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoCaptureSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AdvancedPhotoCaptureSettings[] = L"Windows.Media.Devices.AdvancedPhotoCaptureSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AdvancedPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAdvancedPhotoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AdvancedPhotoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AdvancedPhotoControl[] = L"Windows.Media.Devices.AdvancedPhotoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceController ** Default Interface **
 *    Windows.Media.Devices.IMediaDeviceController
 *    Windows.Media.Devices.IAudioDeviceController2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceController[] = L"Windows.Media.Devices.AudioDeviceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModule ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModule_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModule[] = L"Windows.Media.Devices.AudioDeviceModule";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModuleNotificationEventArgs[] = L"Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.AudioDeviceModulesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Devices.IAudioDeviceModulesManagerFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IAudioDeviceModulesManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModulesManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_AudioDeviceModulesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_AudioDeviceModulesManager[] = L"Windows.Media.Devices.AudioDeviceModulesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.CallControl
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Devices.ICallControlStatics interface starting with version 1.0 of the Windows.Media.Devices.CallControlContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICallControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CallControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CallControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CallControl[] = L"Windows.Media.Devices.CallControl";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionInfo[] = L"Windows.Media.Devices.CameraOcclusionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionState_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionState[] = L"Windows.Media.Devices.CameraOcclusionState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.CameraOcclusionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_CameraOcclusionStateChangedEventArgs[] = L"Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DefaultAudioCaptureDeviceChangedEventArgs[] = L"Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DefaultAudioRenderDeviceChangedEventArgs[] = L"Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDialRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DialRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DialRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DialRequestedEventArgs[] = L"Windows.Media.Devices.DialRequestedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowBounds ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowBounds_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowBounds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowBounds[] = L"Windows.Media.Devices.DigitalWindowBounds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowCapability ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowCapability_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowCapability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowCapability[] = L"Windows.Media.Devices.DigitalWindowCapability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.DigitalWindowControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IDigitalWindowControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_DigitalWindowControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_DigitalWindowControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_DigitalWindowControl[] = L"Windows.Media.Devices.DigitalWindowControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Devices.ExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposureCompensationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposureCompensationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposureCompensationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposureCompensationControl[] = L"Windows.Media.Devices.ExposureCompensationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposureControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposureControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposureControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposureControl[] = L"Windows.Media.Devices.ExposureControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ExposurePriorityVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IExposurePriorityVideoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ExposurePriorityVideoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ExposurePriorityVideoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ExposurePriorityVideoControl[] = L"Windows.Media.Devices.ExposurePriorityVideoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFlashControl ** Default Interface **
 *    Windows.Media.Devices.IFlashControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FlashControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FlashControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FlashControl[] = L"Windows.Media.Devices.FlashControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFocusControl ** Default Interface **
 *    Windows.Media.Devices.IFocusControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FocusControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FocusControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FocusControl[] = L"Windows.Media.Devices.FocusControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.FocusSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IFocusSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_FocusSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_FocusSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_FocusSettings[] = L"Windows.Media.Devices.FocusSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.HdrVideoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IHdrVideoControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_HdrVideoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_HdrVideoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_HdrVideoControl[] = L"Windows.Media.Devices.HdrVideoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.InfraredTorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IInfraredTorchControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Devices_InfraredTorchControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_InfraredTorchControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_InfraredTorchControl[] = L"Windows.Media.Devices.InfraredTorchControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Devices.IsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IIsoSpeedControl ** Default Interface **
 *    Windows.Media.Devices.IIsoSpeedControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_IsoSpeedControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_IsoSpeedControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_IsoSpeedControl[] = L"Windows.Media.Devices.IsoSpeedControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.KeypadPressedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IKeypadPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_KeypadPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_KeypadPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_KeypadPressedEventArgs[] = L"Windows.Media.Devices.KeypadPressedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.LowLagPhotoControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ILowLagPhotoControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_LowLagPhotoControl[] = L"Windows.Media.Devices.LowLagPhotoControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.LowLagPhotoSequenceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ILowLagPhotoSequenceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoSequenceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_LowLagPhotoSequenceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_LowLagPhotoSequenceControl[] = L"Windows.Media.Devices.LowLagPhotoSequenceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Devices.IMediaDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDevice[] = L"Windows.Media.Devices.MediaDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDeviceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IMediaDeviceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDeviceControl[] = L"Windows.Media.Devices.MediaDeviceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.MediaDeviceControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IMediaDeviceControlCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControlCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_MediaDeviceControlCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_MediaDeviceControlCapabilities[] = L"Windows.Media.Devices.MediaDeviceControlCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ModuleCommandResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IModuleCommandResult ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ModuleCommandResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ModuleCommandResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ModuleCommandResult[] = L"Windows.Media.Devices.ModuleCommandResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.OpticalImageStabilizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IOpticalImageStabilizationControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_OpticalImageStabilizationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_OpticalImageStabilizationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_OpticalImageStabilizationControl[] = L"Windows.Media.Devices.OpticalImageStabilizationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.PanelBasedOptimizationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IPanelBasedOptimizationControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Devices_PanelBasedOptimizationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_PanelBasedOptimizationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_PanelBasedOptimizationControl[] = L"Windows.Media.Devices.PanelBasedOptimizationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Devices.PhotoConfirmationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IPhotoConfirmationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_PhotoConfirmationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_PhotoConfirmationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_PhotoConfirmationControl[] = L"Windows.Media.Devices.PhotoConfirmationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RedialRequestedEventArgs
 *
 * Introduced to Windows.Media.Devices.CallControlContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRedialRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RedialRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RedialRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RedialRequestedEventArgs[] = L"Windows.Media.Devices.RedialRequestedEventArgs";
#endif
#endif // WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RegionOfInterest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRegionOfInterest ** Default Interface **
 *    Windows.Media.Devices.IRegionOfInterest2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RegionOfInterest_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RegionOfInterest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RegionOfInterest[] = L"Windows.Media.Devices.RegionOfInterest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.RegionsOfInterestControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IRegionsOfInterestControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_RegionsOfInterestControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_RegionsOfInterestControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_RegionsOfInterestControl[] = L"Windows.Media.Devices.RegionsOfInterestControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.SceneModeControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ISceneModeControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_SceneModeControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_SceneModeControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_SceneModeControl[] = L"Windows.Media.Devices.SceneModeControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.TorchControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.ITorchControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_TorchControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_TorchControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_TorchControl[] = L"Windows.Media.Devices.TorchControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.VideoDeviceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoDeviceController ** Default Interface **
 *    Windows.Media.Devices.IMediaDeviceController
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10
 *    Windows.Media.Devices.IAdvancedVideoCaptureDeviceController11
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoDeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoDeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoDeviceController[] = L"Windows.Media.Devices.VideoDeviceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoDeviceControllerGetDevicePropertyResult[] = L"Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Devices.VideoTemporalDenoisingControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IVideoTemporalDenoisingControl ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Devices_VideoTemporalDenoisingControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_VideoTemporalDenoisingControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_VideoTemporalDenoisingControl[] = L"Windows.Media.Devices.VideoTemporalDenoisingControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Devices.WhiteBalanceControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IWhiteBalanceControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_WhiteBalanceControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_WhiteBalanceControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_WhiteBalanceControl[] = L"Windows.Media.Devices.WhiteBalanceControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ZoomControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IZoomControl ** Default Interface **
 *    Windows.Media.Devices.IZoomControl2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ZoomControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ZoomControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ZoomControl[] = L"Windows.Media.Devices.ZoomControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.ZoomSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.IZoomSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_ZoomSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_ZoomSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_ZoomSettings[] = L"Windows.Media.Devices.ZoomSettings";
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
#endif // __windows2Emedia2Edevices_p_h__

#endif // __windows2Emedia2Edevices_h__
