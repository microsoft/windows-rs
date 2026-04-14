
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
#ifndef __windows2Edevices2Edisplay2Ecore_h__
#define __windows2Edevices2Edisplay2Ecore_h__
#ifndef __windows2Edevices2Edisplay2Ecore_p_h__
#define __windows2Edevices2Edisplay2Ecore_p_h__


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
#include "Windows.Devices.Display.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayAdapter;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter ABI::Windows::Devices::Display::Core::IDisplayAdapter

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayAdapter2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2 ABI::Windows::Devices::Display::Core::IDisplayAdapter2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayAdapterStatics;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics ABI::Windows::Devices::Display::Core::IDisplayAdapterStatics

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayDevice;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice ABI::Windows::Devices::Display::Core::IDisplayDevice

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayDevice2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2 ABI::Windows::Devices::Display::Core::IDisplayDevice2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayDeviceRenderAdapter;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter ABI::Windows::Devices::Display::Core::IDisplayDeviceRenderAdapter

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayFence;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence ABI::Windows::Devices::Display::Core::IDisplayFence

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManager;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager ABI::Windows::Devices::Display::Core::IDisplayManager

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManager2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2 ABI::Windows::Devices::Display::Core::IDisplayManager2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManager3;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3 ABI::Windows::Devices::Display::Core::IDisplayManager3

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerChangedEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs ABI::Windows::Devices::Display::Core::IDisplayManagerChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerDisabledEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs ABI::Windows::Devices::Display::Core::IDisplayManagerDisabledEventArgs

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerEnabledEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs ABI::Windows::Devices::Display::Core::IDisplayManagerEnabledEventArgs

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerPathsFailedOrInvalidatedEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs ABI::Windows::Devices::Display::Core::IDisplayManagerPathsFailedOrInvalidatedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerResultWithState;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayManagerStatics;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics ABI::Windows::Devices::Display::Core::IDisplayManagerStatics

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayModeInfo;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo ABI::Windows::Devices::Display::Core::IDisplayModeInfo

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayModeInfo2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2 ABI::Windows::Devices::Display::Core::IDisplayModeInfo2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayMuxDevice;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice ABI::Windows::Devices::Display::Core::IDisplayMuxDevice

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayMuxDeviceStatics;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics ABI::Windows::Devices::Display::Core::IDisplayMuxDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayPath;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath ABI::Windows::Devices::Display::Core::IDisplayPath

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayPath2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2 ABI::Windows::Devices::Display::Core::IDisplayPath2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayPrimaryDescription;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescription

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayPrimaryDescriptionFactory;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescriptionFactory

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayPrimaryDescriptionStatics;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescriptionStatics

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayScanout;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout ABI::Windows::Devices::Display::Core::IDisplayScanout

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplaySource;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource ABI::Windows::Devices::Display::Core::IDisplaySource

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplaySource2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2 ABI::Windows::Devices::Display::Core::IDisplaySource2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayState;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState ABI::Windows::Devices::Display::Core::IDisplayState

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayStateOperationResult;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult ABI::Windows::Devices::Display::Core::IDisplayStateOperationResult

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplaySurface;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface ABI::Windows::Devices::Display::Core::IDisplaySurface

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTarget;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget ABI::Windows::Devices::Display::Core::IDisplayTarget

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTask;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask ABI::Windows::Devices::Display::Core::IDisplayTask

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTask2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2 ABI::Windows::Devices::Display::Core::IDisplayTask2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTaskPool;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool ABI::Windows::Devices::Display::Core::IDisplayTaskPool

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTaskPool2;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2 ABI::Windows::Devices::Display::Core::IDisplayTaskPool2

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayTaskResult;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult ABI::Windows::Devices::Display::Core::IDisplayTaskResult

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayView;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView ABI::Windows::Devices::Display::Core::IDisplayView

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayWireFormat;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat ABI::Windows::Devices::Display::Core::IDisplayWireFormat

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayWireFormatFactory;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory ABI::Windows::Devices::Display::Core::IDisplayWireFormatFactory

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    interface IDisplayWireFormatStatics;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics ABI::Windows::Devices::Display::Core::IDisplayWireFormatStatics

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayMuxDevice;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("066c3b59-099b-5f37-89eb-1d54fd93e9b3"))
IAsyncOperation<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*, ABI::Windows::Devices::Display::Core::IDisplayMuxDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Display.Core.DisplayMuxDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*> __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61c7b405-e1e1-53b9-a440-4bae01636d23"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*, ABI::Windows::Devices::Display::Core::IDisplayMuxDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Display.Core.DisplayMuxDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayAdapter;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#define DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eeaa87f4-4128-5f75-9eab-d7c8a2e9e14d"))
IIterator<ABI::Windows::Devices::Display::Core::DisplayAdapter*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayAdapter*, ABI::Windows::Devices::Display::Core::IDisplayAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Display.Core.DisplayAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Display::Core::DisplayAdapter*> __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t;
#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#define DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db889bc5-3fd5-5b02-a2ea-d8daab6c33f2"))
IIterable<ABI::Windows::Devices::Display::Core::DisplayAdapter*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayAdapter*, ABI::Windows::Devices::Display::Core::IDisplayAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Display.Core.DisplayAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Display::Core::DisplayAdapter*> __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t;
#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayModeInfo;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#define DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eda19941-13b1-5745-b866-4eb39d0ad62c"))
IIterator<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayModeInfo*, ABI::Windows::Devices::Display::Core::IDisplayModeInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Display.Core.DisplayModeInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t;
#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#define DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dabee849-f1c8-5dab-abe5-effa5110894c"))
IIterable<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayModeInfo*, ABI::Windows::Devices::Display::Core::IDisplayModeInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Display.Core.DisplayModeInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t;
#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayPath;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#define DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("957cdd72-e861-5fe4-9748-216eeb59a9b1"))
IIterator<ABI::Windows::Devices::Display::Core::DisplayPath*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayPath*, ABI::Windows::Devices::Display::Core::IDisplayPath*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Display.Core.DisplayPath>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Display::Core::DisplayPath*> __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t;
#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#define DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4c0352b3-bb9e-52a7-afa7-c946928c12c9"))
IIterable<ABI::Windows::Devices::Display::Core::DisplayPath*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayPath*, ABI::Windows::Devices::Display::Core::IDisplayPath*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Display.Core.DisplayPath>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Display::Core::DisplayPath*> __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t;
#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayTarget;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#define DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("069f9407-81b5-526b-8c63-cdf8ed207ede"))
IIterator<ABI::Windows::Devices::Display::Core::DisplayTarget*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayTarget*, ABI::Windows::Devices::Display::Core::IDisplayTarget*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Display.Core.DisplayTarget>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Display::Core::DisplayTarget*> __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t;
#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#define DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ccb2e527-3a37-50fd-8d28-59e9db8cf9d9"))
IIterable<ABI::Windows::Devices::Display::Core::DisplayTarget*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayTarget*, ABI::Windows::Devices::Display::Core::IDisplayTarget*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Display.Core.DisplayTarget>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Display::Core::DisplayTarget*> __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t;
#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayView;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#define DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a6fbd467-44c0-5ab4-9ad1-25c6eac704d3"))
IIterator<ABI::Windows::Devices::Display::Core::DisplayView*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayView*, ABI::Windows::Devices::Display::Core::IDisplayView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Display.Core.DisplayView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Display::Core::DisplayView*> __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t;
#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#define DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8b18679f-3a2d-5d78-8409-2af682e7433c"))
IIterable<ABI::Windows::Devices::Display::Core::DisplayView*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayView*, ABI::Windows::Devices::Display::Core::IDisplayView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Display.Core.DisplayView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Display::Core::DisplayView*> __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t;
#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000


#ifndef DEF___FIKeyValuePair_2_GUID_IInspectable_USE
#define DEF___FIKeyValuePair_2_GUID_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3bda1540-d089-5a1a-8f0d-94eba8068e58"))
IKeyValuePair<GUID, IInspectable*> : IKeyValuePair_impl<GUID, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<Guid, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<GUID, IInspectable*> __FIKeyValuePair_2_GUID_IInspectable_t;
#define __FIKeyValuePair_2_GUID_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_GUID_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_GUID_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f25059a-0b9a-5f25-9b9e-4b9f1d22ff65"))
IIterator<__FIKeyValuePair_2_GUID_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_GUID_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_GUID_IInspectable*> __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f3b20528-e3b3-5331-b2d0-0c2623aee785"))
IIterable<__FIKeyValuePair_2_GUID_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_GUID_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_GUID_IInspectable*> __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct RectInt32 RectInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE
#define DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1abdf3f6-23f1-55ad-babd-f4cd908406e7"))
IIterator<struct ABI::Windows::Graphics::RectInt32> : IIterator_impl<struct ABI::Windows::Graphics::RectInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.RectInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Graphics::RectInt32> __FIIterator_1_Windows__CGraphics__CRectInt32_t;
#define __FIIterator_1_Windows__CGraphics__CRectInt32 ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CRectInt32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE
#define DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d6222360-b82e-5eed-9eab-2e275b36e47e"))
IIterable<struct ABI::Windows::Graphics::RectInt32> : IIterable_impl<struct ABI::Windows::Graphics::RectInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.RectInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Graphics::RectInt32> __FIIterable_1_Windows__CGraphics__CRectInt32_t;
#define __FIIterable_1_Windows__CGraphics__CRectInt32 ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CRectInt32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIMapView_2_GUID_IInspectable_USE
#define DEF___FIMapView_2_GUID_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e4d2c732-bbc1-5ef4-869f-5007ceb55f6e"))
IMapView<GUID, IInspectable*> : IMapView_impl<GUID, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<Guid, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<GUID, IInspectable*> __FIMapView_2_GUID_IInspectable_t;
#define __FIMapView_2_GUID_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_GUID_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_GUID_IInspectable_USE */



#ifndef DEF___FIMap_2_GUID_IInspectable_USE
#define DEF___FIMap_2_GUID_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5ee3189c-7dbf-5998-ad07-5414fb82567c"))
IMap<GUID, IInspectable*> : IMap_impl<GUID, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<Guid, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<GUID, IInspectable*> __FIMap_2_GUID_IInspectable_t;
#define __FIMap_2_GUID_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_GUID_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_GUID_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#define DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b491b91f-2107-5c1c-aa64-07f39803e64c"))
IVectorView<ABI::Windows::Devices::Display::Core::DisplayAdapter*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayAdapter*, ABI::Windows::Devices::Display::Core::IDisplayAdapter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Display.Core.DisplayAdapter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Display::Core::DisplayAdapter*> __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t;
#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#define DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("339be624-f7dc-562c-b5d2-1b4405bdb7e2"))
IVectorView<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayModeInfo*, ABI::Windows::Devices::Display::Core::IDisplayModeInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Display.Core.DisplayModeInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Display::Core::DisplayModeInfo*> __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t;
#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#define DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb9b91c4-3d7a-52f6-882b-4b850156e12b"))
IVectorView<ABI::Windows::Devices::Display::Core::DisplayPath*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayPath*, ABI::Windows::Devices::Display::Core::IDisplayPath*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Display.Core.DisplayPath>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Display::Core::DisplayPath*> __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t;
#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#define DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5011224d-182d-5d88-a56b-d543c6be3621"))
IVectorView<ABI::Windows::Devices::Display::Core::DisplayTarget*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayTarget*, ABI::Windows::Devices::Display::Core::IDisplayTarget*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Display.Core.DisplayTarget>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Display::Core::DisplayTarget*> __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t;
#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#define DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("442a33f9-7b39-5c9e-84d7-b43cf55171d4"))
IVectorView<ABI::Windows::Devices::Display::Core::DisplayView*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayView*, ABI::Windows::Devices::Display::Core::IDisplayView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Display.Core.DisplayView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Display::Core::DisplayView*> __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t;
#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000


#ifndef DEF___FIReference_1_boolean_USE
#define DEF___FIReference_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c00fd60-2950-5939-a21a-2d12c5a01b8a"))
IReference<bool> : IReference_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<bool> __FIReference_1_boolean_t;
#define __FIReference_1_boolean ABI::Windows::Foundation::__FIReference_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_boolean_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef struct DisplayPresentationRate DisplayPresentationRate;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_USE
#define DEF___FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("08882ddb-a981-5887-b751-106993d66229"))
IReference<struct ABI::Windows::Devices::Display::Core::DisplayPresentationRate> : IReference_impl<struct ABI::Windows::Devices::Display::Core::DisplayPresentationRate>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Devices.Display.Core.DisplayPresentationRate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Devices::Display::Core::DisplayPresentationRate> __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_t;
#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate ABI::Windows::Foundation::__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct SizeInt32 SizeInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIReference_1_Windows__CGraphics__CSizeInt32_USE
#define DEF___FIReference_1_Windows__CGraphics__CSizeInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b77aa86d-2d6e-55f1-8f99-64ac5c05328b"))
IReference<struct ABI::Windows::Graphics::SizeInt32> : IReference_impl<struct ABI::Windows::Graphics::SizeInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Graphics.SizeInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Graphics::SizeInt32> __FIReference_1_Windows__CGraphics__CSizeInt32_t;
#define __FIReference_1_Windows__CGraphics__CSizeInt32 ABI::Windows::Foundation::__FIReference_1_Windows__CGraphics__CSizeInt32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CGraphics__CSizeInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManager;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManagerChangedEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("04d7559d-5bbd-5b73-8782-c2ceea70bd5d"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::IDisplayManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManagerChangedEventArgs*, ABI::Windows::Devices::Display::Core::IDisplayManagerChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplayManager, Windows.Devices.Display.Core.DisplayManagerChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManagerDisabledEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d836ed01-b36e-574c-92d8-86a1b80ba56d"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerDisabledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::IDisplayManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManagerDisabledEventArgs*, ABI::Windows::Devices::Display::Core::IDisplayManagerDisabledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplayManager, Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerDisabledEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManagerEnabledEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a92a0c2b-f309-5bc8-bf69-47c844d6da41"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerEnabledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::IDisplayManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManagerEnabledEventArgs*, ABI::Windows::Devices::Display::Core::IDisplayManagerEnabledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplayManager, Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerEnabledEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManagerPathsFailedOrInvalidatedEventArgs;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28c6cddb-754f-574d-ac7d-be8d773dd4dd"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerPathsFailedOrInvalidatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::IDisplayManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayManagerPathsFailedOrInvalidatedEventArgs*, ABI::Windows::Devices::Display::Core::IDisplayManagerPathsFailedOrInvalidatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplayManager, Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayManager*, ABI::Windows::Devices::Display::Core::DisplayManagerPathsFailedOrInvalidatedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f63ed5f6-c9fb-5876-9e02-a31a9fee1157"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*, ABI::Windows::Devices::Display::Core::IDisplayMuxDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplayMuxDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplayMuxDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplaySource;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("65f431e8-efc4-552f-8404-321531583839"))
ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplaySource*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Display::Core::DisplaySource*, ABI::Windows::Devices::Display::Core::IDisplaySource*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Display.Core.DisplaySource, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Display::Core::DisplaySource*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                class DisplayMonitor;
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                interface IDisplayMonitor;
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor ABI::Windows::Devices::Display::IDisplayMonitor

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                typedef enum DisplayMonitorUsageKind : int DisplayMonitorUsageKind;
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

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
            namespace Numerics {
                typedef struct Rational Rational;
            } /* Numerics */
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
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    typedef struct Direct3DMultisampleDescription Direct3DMultisampleDescription;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXColorSpace : int DirectXColorSpace;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXPixelFormat : int DirectXPixelFormat;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct DisplayAdapterId DisplayAdapterId;
        } /* Graphics */
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
            namespace Display {
                namespace Core {
                    typedef enum DisplayBitsPerChannel : unsigned int DisplayBitsPerChannel;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayDeviceCapability : int DisplayDeviceCapability;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayManagerOptions : unsigned int DisplayManagerOptions;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayManagerResult : int DisplayManagerResult;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayModeQueryOptions : unsigned int DisplayModeQueryOptions;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayPathScaling : int DisplayPathScaling;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayPathStatus : int DisplayPathStatus;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayPresentStatus : int DisplayPresentStatus;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayRotation : int DisplayRotation;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayScanoutOptions : unsigned int DisplayScanoutOptions;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplaySourceStatus : int DisplaySourceStatus;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayStateApplyOptions : unsigned int DisplayStateApplyOptions;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayStateFunctionalizeOptions : unsigned int DisplayStateFunctionalizeOptions;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayStateOperationStatus : int DisplayStateOperationStatus;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayTargetPersistence : int DisplayTargetPersistence;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayTaskSignalKind : int DisplayTaskSignalKind;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayWireFormatColorSpace : int DisplayWireFormatColorSpace;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayWireFormatEotf : int DisplayWireFormatEotf;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayWireFormatHdrMetadata : int DisplayWireFormatHdrMetadata;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    typedef enum DisplayWireFormatPixelEncoding : int DisplayWireFormatPixelEncoding;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayDevice;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayFence;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayManagerResultWithState;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayPrimaryDescription;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayScanout;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayState;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayStateOperationResult;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplaySurface;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayTask;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayTaskPool;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayTaskResult;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    class DisplayWireFormat;
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayBitsPerChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayBitsPerChannel : unsigned int
                    {
                        DisplayBitsPerChannel_None = 0,
                        DisplayBitsPerChannel_Bpc6 = 0x1,
                        DisplayBitsPerChannel_Bpc8 = 0x2,
                        DisplayBitsPerChannel_Bpc10 = 0x4,
                        DisplayBitsPerChannel_Bpc12 = 0x8,
                        DisplayBitsPerChannel_Bpc14 = 0x10,
                        DisplayBitsPerChannel_Bpc16 = 0x20,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayBitsPerChannel)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayDeviceCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayDeviceCapability : int
                    {
                        DisplayDeviceCapability_FlipOverride = 0,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayManagerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayManagerOptions : unsigned int
                    {
                        DisplayManagerOptions_None = 0,
                        DisplayManagerOptions_EnforceSourceOwnership = 0x1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        DisplayManagerOptions_VirtualRefreshRateAware = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayManagerOptions)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayManagerResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayManagerResult : int
                    {
                        DisplayManagerResult_Success = 0,
                        DisplayManagerResult_UnknownFailure = 1,
                        DisplayManagerResult_TargetAccessDenied = 2,
                        DisplayManagerResult_TargetStale = 3,
                        DisplayManagerResult_RemoteSessionNotSupported = 4,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayModeQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayModeQueryOptions : unsigned int
                    {
                        DisplayModeQueryOptions_None = 0,
                        DisplayModeQueryOptions_OnlyPreferredResolution = 0x1,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayModeQueryOptions)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPathScaling
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayPathScaling : int
                    {
                        DisplayPathScaling_Identity = 0,
                        DisplayPathScaling_Centered = 1,
                        DisplayPathScaling_Stretched = 2,
                        DisplayPathScaling_AspectRatioStretched = 3,
                        DisplayPathScaling_Custom = 4,
                        DisplayPathScaling_DriverPreferred = 5,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPathStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayPathStatus : int
                    {
                        DisplayPathStatus_Unknown = 0,
                        DisplayPathStatus_Succeeded = 1,
                        DisplayPathStatus_Pending = 2,
                        DisplayPathStatus_Failed = 3,
                        DisplayPathStatus_FailedAsync = 4,
                        DisplayPathStatus_InvalidatedAsync = 5,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPresentStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayPresentStatus : int
                    {
                        DisplayPresentStatus_Success = 0,
                        DisplayPresentStatus_SourceStatusPreventedPresent = 1,
                        DisplayPresentStatus_ScanoutInvalid = 2,
                        DisplayPresentStatus_SourceInvalid = 3,
                        DisplayPresentStatus_DeviceInvalid = 4,
                        DisplayPresentStatus_UnknownFailure = 5,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayRotation : int
                    {
                        DisplayRotation_None = 0,
                        DisplayRotation_Clockwise90Degrees = 1,
                        DisplayRotation_Clockwise180Degrees = 2,
                        DisplayRotation_Clockwise270Degrees = 3,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayScanoutOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayScanoutOptions : unsigned int
                    {
                        DisplayScanoutOptions_None = 0,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        DisplayScanoutOptions_AllowTearing = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayScanoutOptions)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplaySourceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplaySourceStatus : int
                    {
                        DisplaySourceStatus_Active = 0,
                        DisplaySourceStatus_PoweredOff = 1,
                        DisplaySourceStatus_Invalid = 2,
                        DisplaySourceStatus_OwnedByAnotherDevice = 3,
                        DisplaySourceStatus_Unowned = 4,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateApplyOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayStateApplyOptions : unsigned int
                    {
                        DisplayStateApplyOptions_None = 0,
                        DisplayStateApplyOptions_FailIfStateChanged = 0x1,
                        DisplayStateApplyOptions_ForceReapply = 0x2,
                        DisplayStateApplyOptions_ForceModeEnumeration = 0x4,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayStateApplyOptions)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateFunctionalizeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayStateFunctionalizeOptions : unsigned int
                    {
                        DisplayStateFunctionalizeOptions_None = 0,
                        DisplayStateFunctionalizeOptions_FailIfStateChanged = 0x1,
                        DisplayStateFunctionalizeOptions_ValidateTopologyOnly = 0x2,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(DisplayStateFunctionalizeOptions)
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateOperationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayStateOperationStatus : int
                    {
                        DisplayStateOperationStatus_Success = 0,
                        DisplayStateOperationStatus_PartialFailure = 1,
                        DisplayStateOperationStatus_UnknownFailure = 2,
                        DisplayStateOperationStatus_TargetOwnershipLost = 3,
                        DisplayStateOperationStatus_SystemStateChanged = 4,
                        DisplayStateOperationStatus_TooManyPathsForAdapter = 5,
                        DisplayStateOperationStatus_ModesNotSupported = 6,
                        DisplayStateOperationStatus_RemoteSessionNotSupported = 7,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayTargetPersistence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayTargetPersistence : int
                    {
                        DisplayTargetPersistence_None = 0,
                        DisplayTargetPersistence_BootPersisted = 1,
                        DisplayTargetPersistence_TemporaryPersisted = 2,
                        DisplayTargetPersistence_PathPersisted = 3,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayTaskSignalKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayTaskSignalKind : int
                    {
                        DisplayTaskSignalKind_OnPresentFlipAway = 0,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        DisplayTaskSignalKind_OnPresentFlipTo = 1,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayWireFormatColorSpace : int
                    {
                        DisplayWireFormatColorSpace_BT709 = 0,
                        DisplayWireFormatColorSpace_BT2020 = 1,
                        DisplayWireFormatColorSpace_ProfileDefinedWideColorGamut = 2,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatEotf
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayWireFormatEotf : int
                    {
                        DisplayWireFormatEotf_Sdr = 0,
                        DisplayWireFormatEotf_HdrSmpte2084 = 1,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatHdrMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayWireFormatHdrMetadata : int
                    {
                        DisplayWireFormatHdrMetadata_None = 0,
                        DisplayWireFormatHdrMetadata_Hdr10 = 1,
                        DisplayWireFormatHdrMetadata_Hdr10Plus = 2,
                        DisplayWireFormatHdrMetadata_DolbyVisionLowLatency = 3,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatPixelEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    enum DisplayWireFormatPixelEncoding : int
                    {
                        DisplayWireFormatPixelEncoding_Rgb444 = 0,
                        DisplayWireFormatPixelEncoding_Ycc444 = 1,
                        DisplayWireFormatPixelEncoding_Ycc422 = 2,
                        DisplayWireFormatPixelEncoding_Ycc420 = 3,
                        DisplayWireFormatPixelEncoding_Intensity = 4,
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPresentationRate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    struct DisplayPresentationRate
                    {
                        ABI::Windows::Foundation::Numerics::Rational VerticalSyncRate;
                        INT32 VerticalSyncsPerPresentation;
                    };
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapter[] = L"Windows.Devices.Display.Core.IDisplayAdapter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("a56f5287-f000-5f2e-b5ac-3783a2b69af5")
                    IDisplayAdapter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            ABI::Windows::Graphics::DisplayAdapterId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceInterfacePath(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceCount(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PciVendorId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PciDeviceId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PciSubSystemId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PciRevision(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayAdapter = __uuidof(IDisplayAdapter);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapter2[] = L"Windows.Devices.Display.Core.IDisplayAdapter2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("9ab49b18-b3c7-5546-8374-a9127111edd8")
                    IDisplayAdapter2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsIndirectDisplayDevice(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredRenderAdapter(
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayAdapter2 = __uuidof(IDisplayAdapter2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapterStatics[] = L"Windows.Devices.Display.Core.IDisplayAdapterStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("1dac3cda-481f-5469-8470-82c4ba680a28")
                    IDisplayAdapterStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromId(
                            ABI::Windows::Graphics::DisplayAdapterId id,
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayAdapterStatics = __uuidof(IDisplayAdapterStatics);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDevice[] = L"Windows.Devices.Display.Core.IDisplayDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("a4c9b62c-335f-5731-8cb4-c1ccd4731070")
                    IDisplayDevice : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateScanoutSource(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplaySource** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreatePrimary(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescription* desc,
                            ABI::Windows::Devices::Display::Core::IDisplaySurface** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateTaskPool(
                            ABI::Windows::Devices::Display::Core::IDisplayTaskPool** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreatePeriodicFence(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Foundation::TimeSpan offsetFromVBlank,
                            ABI::Windows::Devices::Display::Core::IDisplayFence** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE WaitForVBlank(
                            ABI::Windows::Devices::Display::Core::IDisplaySource* source
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateSimpleScanout(
                            ABI::Windows::Devices::Display::Core::IDisplaySource* pSource,
                            ABI::Windows::Devices::Display::Core::IDisplaySurface* pSurface,
                            UINT32 SubResourceIndex,
                            UINT32 SyncInterval,
                            ABI::Windows::Devices::Display::Core::IDisplayScanout** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsCapabilitySupported(
                            ABI::Windows::Devices::Display::Core::DisplayDeviceCapability capability,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayDevice = __uuidof(IDisplayDevice);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDevice2[] = L"Windows.Devices.Display.Core.IDisplayDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("3fefe50c-0940-54bd-a02f-f9c7a536ad60")
                    IDisplayDevice2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateSimpleScanoutWithDirtyRectsAndOptions(
                            ABI::Windows::Devices::Display::Core::IDisplaySource* source,
                            ABI::Windows::Devices::Display::Core::IDisplaySurface* surface,
                            UINT32 subresourceIndex,
                            UINT32 syncInterval,
                            __FIIterable_1_Windows__CGraphics__CRectInt32* dirtyRects,
                            ABI::Windows::Devices::Display::Core::DisplayScanoutOptions options,
                            ABI::Windows::Devices::Display::Core::IDisplayScanout** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayDevice2 = __uuidof(IDisplayDevice2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDeviceRenderAdapter[] = L"Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("41c86ce5-b18f-573f-9d59-70463115e4cc")
                    IDisplayDeviceRenderAdapter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RenderAdapterId(
                            ABI::Windows::Graphics::DisplayAdapterId* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayDeviceRenderAdapter = __uuidof(IDisplayDeviceRenderAdapter);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayFence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayFence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayFence[] = L"Windows.Devices.Display.Core.IDisplayFence";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("04dcf9ef-3406-5700-8fec-77eba4c5a74b")
                    IDisplayFence : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDisplayFence = __uuidof(IDisplayFence);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager[] = L"Windows.Devices.Display.Core.IDisplayManager";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("4ed9245b-15ec-56e2-9072-7fe5084a31a7")
                    IDisplayManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentTargets(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentAdapters(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::DisplayManagerResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReleaseTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryReadCurrentStateForAllTargets(
                            ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireTargetsAndReadCurrentState(
                            __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
                            ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireTargetsAndCreateEmptyState(
                            __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
                            ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireTargetsAndCreateSubstate(
                            ABI::Windows::Devices::Display::Core::IDisplayState* existingState,
                            __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
                            ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateDisplayDevice(
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter* adapter,
                            ABI::Windows::Devices::Display::Core::IDisplayDevice** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Enabled(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Enabled(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Disabled(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Disabled(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Changed(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PathsFailedOrInvalidated(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PathsFailedOrInvalidated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManager = __uuidof(IDisplayManager);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager2[] = L"Windows.Devices.Display.Core.IDisplayManager2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("e001ec1e-7eb1-597f-9a30-14d3fe3714cd")
                    IDisplayManager2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryReadCurrentStateForModeQuery(
                            ABI::Windows::Devices::Display::Core::IDisplayManagerResultWithState** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManager2 = __uuidof(IDisplayManager2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager3[] = L"Windows.Devices.Display.Core.IDisplayManager3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("6f14cb89-7f49-587d-93ce-77487cbcb530")
                    IDisplayManager3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateDisplayDeviceForIndirectAdapter(
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter* indirectAdapter,
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter* renderAdapter,
                            ABI::Windows::Devices::Display::Core::IDisplayDevice** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManager3 = __uuidof(IDisplayManager3);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerChangedEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("6abfa285-6cca-5731-bcdc-42e5d2f5c50f")
                    IDisplayManagerChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerChangedEventArgs = __uuidof(IDisplayManagerChangedEventArgs);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerDisabledEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("8726dde4-6793-5973-a11f-5ffbc93fdb90")
                    IDisplayManagerDisabledEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerDisabledEventArgs = __uuidof(IDisplayManagerDisabledEventArgs);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerEnabledEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("f0cf3f6f-42fa-59a2-b297-26e1713de848")
                    IDisplayManagerEnabledEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerEnabledEventArgs = __uuidof(IDisplayManagerEnabledEventArgs);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerPathsFailedOrInvalidatedEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("03a65659-1dec-5c15-b2a2-8fe9129869fe")
                    IDisplayManagerPathsFailedOrInvalidatedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerPathsFailedOrInvalidatedEventArgs = __uuidof(IDisplayManagerPathsFailedOrInvalidatedEventArgs);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerResultWithState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerResultWithState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerResultWithState[] = L"Windows.Devices.Display.Core.IDisplayManagerResultWithState";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("8e656aa6-6614-54be-bfef-4994547f7be1")
                    IDisplayManagerResultWithState : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                            ABI::Windows::Devices::Display::Core::DisplayManagerResult* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedErrorCode(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::Devices::Display::Core::IDisplayState** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerResultWithState = __uuidof(IDisplayManagerResultWithState);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerStatics[] = L"Windows.Devices.Display.Core.IDisplayManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("2b6b9446-b999-5535-9d69-53f092c780a1")
                    IDisplayManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::Devices::Display::Core::DisplayManagerOptions options,
                            ABI::Windows::Devices::Display::Core::IDisplayManager** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayManagerStatics = __uuidof(IDisplayManagerStatics);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayModeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayModeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayModeInfo[] = L"Windows.Devices.Display.Core.IDisplayModeInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("48d513a0-f79b-5a74-a05e-da821f470868")
                    IDisplayModeInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceResolution(
                            ABI::Windows::Graphics::SizeInt32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePixelFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TargetResolution(
                            ABI::Windows::Graphics::SizeInt32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PresentationRate(
                            ABI::Windows::Devices::Display::Core::DisplayPresentationRate* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsInterlaced(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetWireFormatSupportedBitsPerChannel(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatPixelEncoding encoding,
                            ABI::Windows::Devices::Display::Core::DisplayBitsPerChannel* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsWireFormatSupported(
                            ABI::Windows::Devices::Display::Core::IDisplayWireFormat* wireFormat,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayModeInfo = __uuidof(IDisplayModeInfo);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayModeInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayModeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayModeInfo2[] = L"Windows.Devices.Display.Core.IDisplayModeInfo2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("c86fa386-0ddb-5473-bfb0-4b7807b5f909")
                    IDisplayModeInfo2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PhysicalPresentationRate(
                            ABI::Windows::Devices::Display::Core::DisplayPresentationRate* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayModeInfo2 = __uuidof(IDisplayModeInfo2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayMuxDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayMuxDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayMuxDevice[] = L"Windows.Devices.Display.Core.IDisplayMuxDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("d81c4925-83dd-52c9-ab4e-e0833fc75068")
                    IDisplayMuxDevice : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAvailableMuxTargets(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAutomaticTargetSwitchingEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPreferredTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAutomaticTargetSwitching(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Changed(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayMuxDevice = __uuidof(IDisplayMuxDevice);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayMuxDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayMuxDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayMuxDeviceStatics[] = L"Windows.Devices.Display.Core.IDisplayMuxDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("7b37a64a-0465-53da-baee-70028480ced0")
                    IDisplayMuxDeviceStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING deviceInterfaceId,
                            __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayMuxDeviceStatics = __uuidof(IDisplayMuxDeviceStatics);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPath[] = L"Windows.Devices.Display.Core.IDisplayPath";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("b3dfd64a-7460-5cde-811b-d5ae9f3d9f84")
                    IDisplayPath : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_View(
                            ABI::Windows::Devices::Display::Core::IDisplayView** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Target(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Display::Core::DisplayPathStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SourceResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourcePixelFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SourcePixelFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsStereo(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TargetResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TargetResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PresentationRate(
                            __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PresentationRate(
                            __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsInterlaced(
                            __FIReference_1_boolean** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsInterlaced(
                            __FIReference_1_boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WireFormat(
                            ABI::Windows::Devices::Display::Core::IDisplayWireFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WireFormat(
                            ABI::Windows::Devices::Display::Core::IDisplayWireFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rotation(
                            ABI::Windows::Devices::Display::Core::DisplayRotation* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Rotation(
                            ABI::Windows::Devices::Display::Core::DisplayRotation value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Scaling(
                            ABI::Windows::Devices::Display::Core::DisplayPathScaling* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Scaling(
                            ABI::Windows::Devices::Display::Core::DisplayPathScaling value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindModes(
                            ABI::Windows::Devices::Display::Core::DisplayModeQueryOptions flags,
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ApplyPropertiesFromMode(
                            ABI::Windows::Devices::Display::Core::IDisplayModeInfo* modeResult
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMap_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayPath = __uuidof(IDisplayPath);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPath2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPath2[] = L"Windows.Devices.Display.Core.IDisplayPath2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("f32459c5-e994-570b-9ec8-ef42c35a8547")
                    IDisplayPath2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PhysicalPresentationRate(
                            __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PhysicalPresentationRate(
                            __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayPath2 = __uuidof(IDisplayPath2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescription[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescription";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("872591d2-d533-50ff-a85e-06696194b77c")
                    IDisplayPrimaryDescription : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Width(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Height(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Format(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColorSpace(
                            ABI::Windows::Graphics::DirectX::DirectXColorSpace* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MultisampleDescription(
                            ABI::Windows::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayPrimaryDescription = __uuidof(IDisplayPrimaryDescription);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescriptionFactory[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("1a6aff7b-3637-5c46-b479-76d576216e65")
                    IDisplayPrimaryDescriptionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 width,
                            UINT32 height,
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                            ABI::Windows::Graphics::DirectX::DirectXColorSpace colorSpace,
                            boolean isStereo,
                            ABI::Windows::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription multisampleDescription,
                            ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescription** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayPrimaryDescriptionFactory = __uuidof(IDisplayPrimaryDescriptionFactory);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescriptionStatics[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("e60e4cfb-36c9-56dd-8fa1-6ff8c4e0ff07")
                    IDisplayPrimaryDescriptionStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateWithProperties(
                            __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* extraProperties,
                            UINT32 width,
                            UINT32 height,
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                            ABI::Windows::Graphics::DirectX::DirectXColorSpace colorSpace,
                            boolean isStereo,
                            ABI::Windows::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription multisampleDescription,
                            ABI::Windows::Devices::Display::Core::IDisplayPrimaryDescription** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayPrimaryDescriptionStatics = __uuidof(IDisplayPrimaryDescriptionStatics);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayScanout
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayScanout
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayScanout[] = L"Windows.Devices.Display.Core.IDisplayScanout";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("e3051828-1ba5-50e7-8a39-bb1fd2f4f8b9")
                    IDisplayScanout : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDisplayScanout = __uuidof(IDisplayScanout);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySource[] = L"Windows.Devices.Display.Core.IDisplaySource";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("ecd15fc1-eadc-51bc-971d-3bc628db2dd4")
                    IDisplaySource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AdapterId(
                            ABI::Windows::Graphics::DisplayAdapterId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetMetadata(
                            GUID Key,
                            ABI::Windows::Storage::Streams::IBuffer** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplaySource = __uuidof(IDisplaySource);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySource2[] = L"Windows.Devices.Display.Core.IDisplaySource2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("71e18952-b321-5af4-bfe8-03fbea31e40d")
                    IDisplaySource2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Display::Core::DisplaySourceStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplaySource2 = __uuidof(IDisplaySource2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayState[] = L"Windows.Devices.Display.Core.IDisplayState";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("08129321-11b5-5cb2-99f8-e90b479a8a1d")
                    IDisplayState : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStale(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Targets(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Views(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMap_2_GUID_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayPath** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConnectTargetToView(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayView* view,
                            ABI::Windows::Devices::Display::Core::IDisplayPath** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CanConnectTargetToView(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayView* view,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetViewForTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayView** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPathForTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target,
                            ABI::Windows::Devices::Display::Core::IDisplayPath** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DisconnectTarget(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* target
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryFunctionalize(
                            ABI::Windows::Devices::Display::Core::DisplayStateFunctionalizeOptions options,
                            ABI::Windows::Devices::Display::Core::IDisplayStateOperationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryApply(
                            ABI::Windows::Devices::Display::Core::DisplayStateApplyOptions options,
                            ABI::Windows::Devices::Display::Core::IDisplayStateOperationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Clone(
                            ABI::Windows::Devices::Display::Core::IDisplayState** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayState = __uuidof(IDisplayState);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayStateOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayStateOperationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayStateOperationResult[] = L"Windows.Devices.Display.Core.IDisplayStateOperationResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("fcadbfdf-dc27-5638-b7f2-ebdfa4f7ea93")
                    IDisplayStateOperationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Display::Core::DisplayStateOperationStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedErrorCode(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayStateOperationResult = __uuidof(IDisplayStateOperationResult);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySurface
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySurface[] = L"Windows.Devices.Display.Core.IDisplaySurface";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("594f6cc6-139a-56d6-a4b1-15fe2cb76adb")
                    IDisplaySurface : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDisplaySurface = __uuidof(IDisplaySurface);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTarget[] = L"Windows.Devices.Display.Core.IDisplayTarget";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("aec57c6f-47b4-546b-987c-e73fa791fe3a")
                    IDisplayTarget : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Adapter(
                            ABI::Windows::Devices::Display::Core::IDisplayAdapter** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceInterfacePath(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AdapterRelativeId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsConnected(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsVirtualModeEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsVirtualTopologyEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UsageKind(
                            ABI::Windows::Devices::Display::DisplayMonitorUsageKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MonitorPersistence(
                            ABI::Windows::Devices::Display::Core::DisplayTargetPersistence* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StableMonitorId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetMonitor(
                            ABI::Windows::Devices::Display::IDisplayMonitor** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStale(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsSame(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* otherTarget,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsEqual(
                            ABI::Windows::Devices::Display::Core::IDisplayTarget* otherTarget,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTarget = __uuidof(IDisplayTarget);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTask[] = L"Windows.Devices.Display.Core.IDisplayTask";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("5e087448-135b-5bb0-bf63-637f84227c7a")
                    IDisplayTask : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetScanout(
                            ABI::Windows::Devices::Display::Core::IDisplayScanout* scanout
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetWait(
                            ABI::Windows::Devices::Display::Core::IDisplayFence* readyFence,
                            UINT64 readyFenceValue
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTask = __uuidof(IDisplayTask);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTask2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTask2[] = L"Windows.Devices.Display.Core.IDisplayTask2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("0957ea19-bd55-55de-9267-c97b61e71c37")
                    IDisplayTask2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetSignal(
                            ABI::Windows::Devices::Display::Core::DisplayTaskSignalKind signalKind,
                            ABI::Windows::Devices::Display::Core::IDisplayFence* fence
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTask2 = __uuidof(IDisplayTask2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskPool[] = L"Windows.Devices.Display.Core.IDisplayTaskPool";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("c676253d-237d-5548-aafa-3e517fefef1c")
                    IDisplayTaskPool : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateTask(
                            ABI::Windows::Devices::Display::Core::IDisplayTask** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        DEPRECATED("Use TryExecuteTask instead of ExecuteTask. For more info see MSDN")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                        virtual HRESULT STDMETHODCALLTYPE ExecuteTask(
                            ABI::Windows::Devices::Display::Core::IDisplayTask* task
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTaskPool = __uuidof(IDisplayTaskPool);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskPool2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskPool2[] = L"Windows.Devices.Display.Core.IDisplayTaskPool2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("46b879b6-5d17-5955-a872-eb38003db586")
                    IDisplayTaskPool2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryExecuteTask(
                            ABI::Windows::Devices::Display::Core::IDisplayTask* task,
                            ABI::Windows::Devices::Display::Core::IDisplayTaskResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTaskPool2 = __uuidof(IDisplayTaskPool2);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskResult[] = L"Windows.Devices.Display.Core.IDisplayTaskResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("6fbc7d67-f9b1-55e0-9d88-d3a5197a3f59")
                    IDisplayTaskResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PresentStatus(
                            ABI::Windows::Devices::Display::Core::DisplayPresentStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PresentId(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceStatus(
                            ABI::Windows::Devices::Display::Core::DisplaySourceStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayTaskResult = __uuidof(IDisplayTaskResult);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayView[] = L"Windows.Devices.Display.Core.IDisplayView";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("b0c98ca1-b759-5b59-b1ad-f0786aa9e53d")
                    IDisplayView : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Paths(
                            __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContentResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContentResolution(
                            __FIReference_1_Windows__CGraphics__CSizeInt32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPrimaryPath(
                            ABI::Windows::Devices::Display::Core::IDisplayPath* path
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMap_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayView = __uuidof(IDisplayView);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormat[] = L"Windows.Devices.Display.Core.IDisplayWireFormat";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("1acc967d-872c-5a38-bbb9-1d4872b76255")
                    IDisplayWireFormat : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PixelEncoding(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatPixelEncoding* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BitsPerChannel(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColorSpace(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatColorSpace* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Eotf(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatEotf* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HdrMetadata(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatHdrMetadata* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayWireFormat = __uuidof(IDisplayWireFormat);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormatFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormatFactory[] = L"Windows.Devices.Display.Core.IDisplayWireFormatFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("b2efc8d5-09d6-55e6-ad22-9014b3d25229")
                    IDisplayWireFormatFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatPixelEncoding pixelEncoding,
                            INT32 bitsPerChannel,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatColorSpace colorSpace,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatEotf eotf,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatHdrMetadata hdrMetadata,
                            ABI::Windows::Devices::Display::Core::IDisplayWireFormat** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayWireFormatFactory = __uuidof(IDisplayWireFormatFactory);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormatStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormatStatics[] = L"Windows.Devices.Display.Core.IDisplayWireFormatStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("c575a22d-c3e6-5f7a-bdfb-87c6ab8661d5")
                    IDisplayWireFormatStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateWithProperties(
                            __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* extraProperties,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatPixelEncoding pixelEncoding,
                            INT32 bitsPerChannel,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatColorSpace colorSpace,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatEotf eotf,
                            ABI::Windows::Devices::Display::Core::DisplayWireFormatHdrMetadata hdrMetadata,
                            ABI::Windows::Devices::Display::Core::IDisplayWireFormat** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDisplayWireFormatStatics = __uuidof(IDisplayWireFormatStatics);
                } /* Core */
            } /* Display */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayAdapterStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayAdapter ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayAdapter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayAdapter[] = L"Windows.Devices.Display.Core.DisplayAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayDevice ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayDevice2
 *    Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayDevice[] = L"Windows.Devices.Display.Core.DisplayDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayFence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayFence ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayFence_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayFence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayFence[] = L"Windows.Devices.Display.Core.DisplayFence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayManagerStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManager ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayManager2
 *    Windows.Devices.Display.Core.IDisplayManager3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManager_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManager[] = L"Windows.Devices.Display.Core.DisplayManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerResultWithState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerResultWithState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerResultWithState_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerResultWithState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerResultWithState[] = L"Windows.Devices.Display.Core.DisplayManagerResultWithState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayModeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayModeInfo ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayModeInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayModeInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayModeInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayModeInfo[] = L"Windows.Devices.Display.Core.DisplayModeInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayMuxDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayMuxDeviceStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayMuxDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayMuxDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayMuxDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayMuxDevice[] = L"Windows.Devices.Display.Core.DisplayMuxDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayPath ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayPath2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPath_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPath_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayPath[] = L"Windows.Devices.Display.Core.DisplayPath";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayPrimaryDescription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPrimaryDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPrimaryDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayPrimaryDescription[] = L"Windows.Devices.Display.Core.DisplayPrimaryDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayScanout
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayScanout ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayScanout_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayScanout_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayScanout[] = L"Windows.Devices.Display.Core.DisplayScanout";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplaySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplaySource ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplaySource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySource_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplaySource[] = L"Windows.Devices.Display.Core.DisplaySource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayState_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayState[] = L"Windows.Devices.Display.Core.DisplayState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayStateOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayStateOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayStateOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayStateOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayStateOperationResult[] = L"Windows.Devices.Display.Core.DisplayStateOperationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplaySurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplaySurface ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySurface_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySurface_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplaySurface[] = L"Windows.Devices.Display.Core.DisplaySurface";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTarget ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTarget_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTarget[] = L"Windows.Devices.Display.Core.DisplayTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTask ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayTask2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTask_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTask[] = L"Windows.Devices.Display.Core.DisplayTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTaskPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTaskPool ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayTaskPool2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskPool_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskPool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTaskPool[] = L"Windows.Devices.Display.Core.DisplayTaskPool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTaskResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTaskResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTaskResult[] = L"Windows.Devices.Display.Core.DisplayTaskResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayView ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayView_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayView[] = L"Windows.Devices.Display.Core.DisplayView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayWireFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Display.Core.IDisplayWireFormatFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayWireFormatStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayWireFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayWireFormat_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayWireFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayWireFormat[] = L"Windows.Devices.Display.Core.DisplayWireFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2 __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

typedef struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl;

interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

typedef struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl;

interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

typedef struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl;

interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

typedef struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl;

interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

typedef struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl;

interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

typedef struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayPath** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl;

interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

typedef struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl;

interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

typedef struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl;

interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

typedef struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl;

interface __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

typedef struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        __FIIterator_1_Windows__CDevices__CDisplay__CCore__CDisplayView** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl;

interface __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if !defined(____FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_GUID_IInspectable __FIKeyValuePair_2_GUID_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_GUID_IInspectable;

typedef struct __FIKeyValuePair_2_GUID_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_GUID_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_GUID_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_GUID_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_GUID_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_GUID_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_GUID_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_GUID_IInspectable* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_GUID_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_GUID_IInspectableVtbl;

interface __FIKeyValuePair_2_GUID_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_GUID_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_GUID_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_GUID_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_GUID_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_GUID_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_GUID_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_GUID_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_GUID_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_GUID_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_GUID_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_GUID_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        __FIKeyValuePair_2_GUID_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_GUID_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_GUID_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_GUID_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_GUID_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_GUID_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_GUID_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_GUID_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_GUID_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_GUID_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CRectInt32 __x_ABI_CWindows_CGraphics_CRectInt32;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CRectInt32 __FIIterator_1_Windows__CGraphics__CRectInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CRectInt32;

typedef struct __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CRectInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CRectInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl;

interface __FIIterator_1_Windows__CGraphics__CRectInt32
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CRectInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CRectInt32 __FIIterable_1_Windows__CGraphics__CRectInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CRectInt32;

typedef struct __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CRectInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CRectInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        __FIIterator_1_Windows__CGraphics__CRectInt32** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl;

interface __FIIterable_1_Windows__CGraphics__CRectInt32
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CRectInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIMapView_2_GUID_IInspectable __FIMapView_2_GUID_IInspectable;

#if !defined(____FIMapView_2_GUID_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_GUID_IInspectable __FIMapView_2_GUID_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_GUID_IInspectable;

typedef struct __FIMapView_2_GUID_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_GUID_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_GUID_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_GUID_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_GUID_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_GUID_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_GUID_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_GUID_IInspectable* This,
        GUID key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_GUID_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_GUID_IInspectable* This,
        GUID key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_GUID_IInspectable* This,
        __FIMapView_2_GUID_IInspectable** first,
        __FIMapView_2_GUID_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_GUID_IInspectableVtbl;

interface __FIMapView_2_GUID_IInspectable
{
    CONST_VTBL struct __FIMapView_2_GUID_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_GUID_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_GUID_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_GUID_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_GUID_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_GUID_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_GUID_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_GUID_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_GUID_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_GUID_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_GUID_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_GUID_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIMap_2_GUID_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_GUID_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_GUID_IInspectable __FIMap_2_GUID_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_GUID_IInspectable;

typedef struct __FIMap_2_GUID_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_GUID_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_GUID_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_GUID_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_GUID_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_GUID_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_GUID_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_GUID_IInspectable* This,
        GUID key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_GUID_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_GUID_IInspectable* This,
        GUID key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_GUID_IInspectable* This,
        __FIMapView_2_GUID_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_GUID_IInspectable* This,
        GUID key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_GUID_IInspectable* This,
        GUID key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_GUID_IInspectable* This);

    END_INTERFACE
} __FIMap_2_GUID_IInspectableVtbl;

interface __FIMap_2_GUID_IInspectable
{
    CONST_VTBL struct __FIMap_2_GUID_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_GUID_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_GUID_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_GUID_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_GUID_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_GUID_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_GUID_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_GUID_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_GUID_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_GUID_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_GUID_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_GUID_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_GUID_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_GUID_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_GUID_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter;

typedef struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl;

interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo;

typedef struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl;

interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath;

typedef struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl;

interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget;

typedef struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl;

interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView;

typedef struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl;

interface __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if !defined(____FIReference_1_boolean_INTERFACE_DEFINED__)
#define ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef interface __FIReference_1_boolean __FIReference_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_boolean;

typedef struct __FIReference_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIReference_1_booleanVtbl;

interface __FIReference_1_boolean
{
    CONST_VTBL struct __FIReference_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_boolean_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate;

typedef struct __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* This,
        struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate* result);

    END_INTERFACE
} __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRateVtbl;

interface __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate
{
    CONST_VTBL struct __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef struct __x_ABI_CWindows_CGraphics_CSizeInt32 __x_ABI_CWindows_CGraphics_CSizeInt32;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIReference_1_Windows__CGraphics__CSizeInt32_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CGraphics__CSizeInt32_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CGraphics__CSizeInt32 __FIReference_1_Windows__CGraphics__CSizeInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CGraphics__CSizeInt32;

typedef struct __FIReference_1_Windows__CGraphics__CSizeInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CGraphics__CSizeInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CGraphics__CSizeInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CGraphics__CSizeInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CGraphics__CSizeInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CGraphics__CSizeInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CGraphics__CSizeInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CGraphics__CSizeInt32* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* result);

    END_INTERFACE
} __FIReference_1_Windows__CGraphics__CSizeInt32Vtbl;

interface __FIReference_1_Windows__CGraphics__CSizeInt32
{
    CONST_VTBL struct __FIReference_1_Windows__CGraphics__CSizeInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CGraphics__CSizeInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CGraphics__CSizeInt32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CGraphics__CSizeInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* sender,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* sender,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* sender,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* sender,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor __x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor;

#endif // ____x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CDisplayMonitorUsageKind __x_ABI_CWindows_CDevices_CDisplay_CDisplayMonitorUsageKind;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CRational __x_ABI_CWindows_CFoundation_CNumerics_CRational;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription;

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace;

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

typedef struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId __x_ABI_CWindows_CGraphics_CDisplayAdapterId;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayBitsPerChannel __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayBitsPerChannel;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayDeviceCapability __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayDeviceCapability;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerOptions __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerOptions;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerResult __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerResult;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayModeQueryOptions __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayModeQueryOptions;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathScaling __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathScaling;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathStatus __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathStatus;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentStatus __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentStatus;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayRotation __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayRotation;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayScanoutOptions __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayScanoutOptions;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplaySourceStatus __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplaySourceStatus;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateApplyOptions __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateApplyOptions;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateFunctionalizeOptions __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateFunctionalizeOptions;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateOperationStatus __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateOperationStatus;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTargetPersistence __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTargetPersistence;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTaskSignalKind __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTaskSignalKind;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata;

typedef enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding;

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayBitsPerChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayBitsPerChannel
{
    DisplayBitsPerChannel_None = 0,
    DisplayBitsPerChannel_Bpc6 = 0x1,
    DisplayBitsPerChannel_Bpc8 = 0x2,
    DisplayBitsPerChannel_Bpc10 = 0x4,
    DisplayBitsPerChannel_Bpc12 = 0x8,
    DisplayBitsPerChannel_Bpc14 = 0x10,
    DisplayBitsPerChannel_Bpc16 = 0x20,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayDeviceCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayDeviceCapability
{
    DisplayDeviceCapability_FlipOverride = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayManagerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerOptions
{
    DisplayManagerOptions_None = 0,
    DisplayManagerOptions_EnforceSourceOwnership = 0x1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DisplayManagerOptions_VirtualRefreshRateAware = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayManagerResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerResult
{
    DisplayManagerResult_Success = 0,
    DisplayManagerResult_UnknownFailure = 1,
    DisplayManagerResult_TargetAccessDenied = 2,
    DisplayManagerResult_TargetStale = 3,
    DisplayManagerResult_RemoteSessionNotSupported = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayModeQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayModeQueryOptions
{
    DisplayModeQueryOptions_None = 0,
    DisplayModeQueryOptions_OnlyPreferredResolution = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPathScaling
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathScaling
{
    DisplayPathScaling_Identity = 0,
    DisplayPathScaling_Centered = 1,
    DisplayPathScaling_Stretched = 2,
    DisplayPathScaling_AspectRatioStretched = 3,
    DisplayPathScaling_Custom = 4,
    DisplayPathScaling_DriverPreferred = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPathStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathStatus
{
    DisplayPathStatus_Unknown = 0,
    DisplayPathStatus_Succeeded = 1,
    DisplayPathStatus_Pending = 2,
    DisplayPathStatus_Failed = 3,
    DisplayPathStatus_FailedAsync = 4,
    DisplayPathStatus_InvalidatedAsync = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPresentStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentStatus
{
    DisplayPresentStatus_Success = 0,
    DisplayPresentStatus_SourceStatusPreventedPresent = 1,
    DisplayPresentStatus_ScanoutInvalid = 2,
    DisplayPresentStatus_SourceInvalid = 3,
    DisplayPresentStatus_DeviceInvalid = 4,
    DisplayPresentStatus_UnknownFailure = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayRotation
{
    DisplayRotation_None = 0,
    DisplayRotation_Clockwise90Degrees = 1,
    DisplayRotation_Clockwise180Degrees = 2,
    DisplayRotation_Clockwise270Degrees = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayScanoutOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayScanoutOptions
{
    DisplayScanoutOptions_None = 0,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DisplayScanoutOptions_AllowTearing = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplaySourceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplaySourceStatus
{
    DisplaySourceStatus_Active = 0,
    DisplaySourceStatus_PoweredOff = 1,
    DisplaySourceStatus_Invalid = 2,
    DisplaySourceStatus_OwnedByAnotherDevice = 3,
    DisplaySourceStatus_Unowned = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateApplyOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateApplyOptions
{
    DisplayStateApplyOptions_None = 0,
    DisplayStateApplyOptions_FailIfStateChanged = 0x1,
    DisplayStateApplyOptions_ForceReapply = 0x2,
    DisplayStateApplyOptions_ForceModeEnumeration = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateFunctionalizeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateFunctionalizeOptions
{
    DisplayStateFunctionalizeOptions_None = 0,
    DisplayStateFunctionalizeOptions_FailIfStateChanged = 0x1,
    DisplayStateFunctionalizeOptions_ValidateTopologyOnly = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayStateOperationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateOperationStatus
{
    DisplayStateOperationStatus_Success = 0,
    DisplayStateOperationStatus_PartialFailure = 1,
    DisplayStateOperationStatus_UnknownFailure = 2,
    DisplayStateOperationStatus_TargetOwnershipLost = 3,
    DisplayStateOperationStatus_SystemStateChanged = 4,
    DisplayStateOperationStatus_TooManyPathsForAdapter = 5,
    DisplayStateOperationStatus_ModesNotSupported = 6,
    DisplayStateOperationStatus_RemoteSessionNotSupported = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayTargetPersistence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTargetPersistence
{
    DisplayTargetPersistence_None = 0,
    DisplayTargetPersistence_BootPersisted = 1,
    DisplayTargetPersistence_TemporaryPersisted = 2,
    DisplayTargetPersistence_PathPersisted = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayTaskSignalKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTaskSignalKind
{
    DisplayTaskSignalKind_OnPresentFlipAway = 0,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DisplayTaskSignalKind_OnPresentFlipTo = 1,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace
{
    DisplayWireFormatColorSpace_BT709 = 0,
    DisplayWireFormatColorSpace_BT2020 = 1,
    DisplayWireFormatColorSpace_ProfileDefinedWideColorGamut = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatEotf
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf
{
    DisplayWireFormatEotf_Sdr = 0,
    DisplayWireFormatEotf_HdrSmpte2084 = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatHdrMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata
{
    DisplayWireFormatHdrMetadata_None = 0,
    DisplayWireFormatHdrMetadata_Hdr10 = 1,
    DisplayWireFormatHdrMetadata_Hdr10Plus = 2,
    DisplayWireFormatHdrMetadata_DolbyVisionLowLatency = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayWireFormatPixelEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding
{
    DisplayWireFormatPixelEncoding_Rgb444 = 0,
    DisplayWireFormatPixelEncoding_Ycc444 = 1,
    DisplayWireFormatPixelEncoding_Ycc422 = 2,
    DisplayWireFormatPixelEncoding_Ycc420 = 3,
    DisplayWireFormatPixelEncoding_Intensity = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Display.Core.DisplayPresentationRate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CRational VerticalSyncRate;
    INT32 VerticalSyncsPerPresentation;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapter[] = L"Windows.Devices.Display.Core.IDisplayAdapter";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInterfacePath)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceCount)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PciVendorId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PciDeviceId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PciSubSystemId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PciRevision)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* This,
        __FIMapView_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_DeviceInterfacePath(This, value) \
    ((This)->lpVtbl->get_DeviceInterfacePath(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_SourceCount(This, value) \
    ((This)->lpVtbl->get_SourceCount(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_PciVendorId(This, value) \
    ((This)->lpVtbl->get_PciVendorId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_PciDeviceId(This, value) \
    ((This)->lpVtbl->get_PciDeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_PciSubSystemId(This, value) \
    ((This)->lpVtbl->get_PciSubSystemId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_PciRevision(This, value) \
    ((This)->lpVtbl->get_PciRevision(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapter2[] = L"Windows.Devices.Display.Core.IDisplayAdapter2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsIndirectDisplayDevice)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredRenderAdapter)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_get_IsIndirectDisplayDevice(This, value) \
    ((This)->lpVtbl->get_IsIndirectDisplayDevice(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_get_PreferredRenderAdapter(This, value) \
    ((This)->lpVtbl->get_PreferredRenderAdapter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayAdapterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayAdapter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayAdapterStatics[] = L"Windows.Devices.Display.Core.IDisplayAdapterStatics";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId id,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_FromId(This, id, result) \
    ((This)->lpVtbl->FromId(This, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDevice[] = L"Windows.Devices.Display.Core.IDisplayDevice";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateScanoutSource)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource** result);
    HRESULT (STDMETHODCALLTYPE* CreatePrimary)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* desc,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface** result);
    HRESULT (STDMETHODCALLTYPE* CreateTaskPool)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool** result);
    HRESULT (STDMETHODCALLTYPE* CreatePeriodicFence)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan offsetFromVBlank,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence** result);
    HRESULT (STDMETHODCALLTYPE* WaitForVBlank)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* source);
    HRESULT (STDMETHODCALLTYPE* CreateSimpleScanout)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* pSource,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* pSurface,
        UINT32 SubResourceIndex,
        UINT32 SyncInterval,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout** result);
    HRESULT (STDMETHODCALLTYPE* IsCapabilitySupported)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayDeviceCapability capability,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_CreateScanoutSource(This, target, result) \
    ((This)->lpVtbl->CreateScanoutSource(This, target, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_CreatePrimary(This, target, desc, result) \
    ((This)->lpVtbl->CreatePrimary(This, target, desc, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_CreateTaskPool(This, result) \
    ((This)->lpVtbl->CreateTaskPool(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_CreatePeriodicFence(This, target, offsetFromVBlank, result) \
    ((This)->lpVtbl->CreatePeriodicFence(This, target, offsetFromVBlank, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_WaitForVBlank(This, source) \
    ((This)->lpVtbl->WaitForVBlank(This, source))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_CreateSimpleScanout(This, pSource, pSurface, SubResourceIndex, SyncInterval, result) \
    ((This)->lpVtbl->CreateSimpleScanout(This, pSource, pSurface, SubResourceIndex, SyncInterval, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_IsCapabilitySupported(This, capability, result) \
    ((This)->lpVtbl->IsCapabilitySupported(This, capability, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDevice2[] = L"Windows.Devices.Display.Core.IDisplayDevice2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSimpleScanoutWithDirtyRectsAndOptions)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* source,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* surface,
        UINT32 subresourceIndex,
        UINT32 syncInterval,
        __FIIterable_1_Windows__CGraphics__CRectInt32* dirtyRects,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayScanoutOptions options,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_CreateSimpleScanoutWithDirtyRectsAndOptions(This, source, surface, subresourceIndex, syncInterval, dirtyRects, options, result) \
    ((This)->lpVtbl->CreateSimpleScanoutWithDirtyRectsAndOptions(This, source, surface, subresourceIndex, syncInterval, dirtyRects, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayDeviceRenderAdapter[] = L"Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RenderAdapterId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapterVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_get_RenderAdapterId(This, value) \
    ((This)->lpVtbl->get_RenderAdapterId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDeviceRenderAdapter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayFence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayFence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayFence[] = L"Windows.Devices.Display.Core.IDisplayFence";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFenceVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager[] = L"Windows.Devices.Display.Core.IDisplayManager";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentTargets)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** result);
    HRESULT (STDMETHODCALLTYPE* GetCurrentAdapters)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayAdapter** result);
    HRESULT (STDMETHODCALLTYPE* TryAcquireTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerResult* result);
    HRESULT (STDMETHODCALLTYPE* ReleaseTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target);
    HRESULT (STDMETHODCALLTYPE* TryReadCurrentStateForAllTargets)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState** result);
    HRESULT (STDMETHODCALLTYPE* TryAcquireTargetsAndReadCurrentState)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState** result);
    HRESULT (STDMETHODCALLTYPE* TryAcquireTargetsAndCreateEmptyState)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState** result);
    HRESULT (STDMETHODCALLTYPE* TryAcquireTargetsAndCreateSubstate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* existingState,
        __FIIterable_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget* targets,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState** result);
    HRESULT (STDMETHODCALLTYPE* CreateDisplayDevice)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* adapter,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice** result);
    HRESULT (STDMETHODCALLTYPE* add_Enabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerEnabledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Enabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Disabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerDisabledEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Disabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PathsFailedOrInvalidated)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayManager_Windows__CDevices__CDisplay__CCore__CDisplayManagerPathsFailedOrInvalidatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PathsFailedOrInvalidated)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_GetCurrentTargets(This, result) \
    ((This)->lpVtbl->GetCurrentTargets(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_GetCurrentAdapters(This, result) \
    ((This)->lpVtbl->GetCurrentAdapters(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_TryAcquireTarget(This, target, result) \
    ((This)->lpVtbl->TryAcquireTarget(This, target, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_ReleaseTarget(This, target) \
    ((This)->lpVtbl->ReleaseTarget(This, target))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_TryReadCurrentStateForAllTargets(This, result) \
    ((This)->lpVtbl->TryReadCurrentStateForAllTargets(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_TryAcquireTargetsAndReadCurrentState(This, targets, result) \
    ((This)->lpVtbl->TryAcquireTargetsAndReadCurrentState(This, targets, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_TryAcquireTargetsAndCreateEmptyState(This, targets, result) \
    ((This)->lpVtbl->TryAcquireTargetsAndCreateEmptyState(This, targets, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_TryAcquireTargetsAndCreateSubstate(This, existingState, targets, result) \
    ((This)->lpVtbl->TryAcquireTargetsAndCreateSubstate(This, existingState, targets, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_CreateDisplayDevice(This, adapter, result) \
    ((This)->lpVtbl->CreateDisplayDevice(This, adapter, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_add_Enabled(This, handler, token) \
    ((This)->lpVtbl->add_Enabled(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_remove_Enabled(This, token) \
    ((This)->lpVtbl->remove_Enabled(This, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_add_Disabled(This, handler, token) \
    ((This)->lpVtbl->add_Disabled(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_remove_Disabled(This, token) \
    ((This)->lpVtbl->remove_Disabled(This, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_add_PathsFailedOrInvalidated(This, handler, token) \
    ((This)->lpVtbl->add_PathsFailedOrInvalidated(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_remove_PathsFailedOrInvalidated(This, token) \
    ((This)->lpVtbl->remove_PathsFailedOrInvalidated(This, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager2[] = L"Windows.Devices.Display.Core.IDisplayManager2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryReadCurrentStateForModeQuery)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_TryReadCurrentStateForModeQuery(This, result) \
    ((This)->lpVtbl->TryReadCurrentStateForModeQuery(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManager3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManager3[] = L"Windows.Devices.Display.Core.IDisplayManager3";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDisplayDeviceForIndirectAdapter)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* indirectAdapter,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter* renderAdapter,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayDevice** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_CreateDisplayDeviceForIndirectAdapter(This, indirectAdapter, renderAdapter, result) \
    ((This)->lpVtbl->CreateDisplayDeviceForIndirectAdapter(This, indirectAdapter, renderAdapter, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerChangedEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerDisabledEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerDisabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerEnabledEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerEnabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerPathsFailedOrInvalidatedEventArgs[] = L"Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerPathsFailedOrInvalidatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerResultWithState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManagerResultWithState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerResultWithState[] = L"Windows.Devices.Display.Core.IDisplayManagerResultWithState";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerResult* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedErrorCode)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithStateVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_get_ExtendedErrorCode(This, value) \
    ((This)->lpVtbl->get_ExtendedErrorCode(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerResultWithState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayManagerStatics[] = L"Windows.Devices.Display.Core.IDisplayManagerStatics";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayManagerOptions options,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_Create(This, options, result) \
    ((This)->lpVtbl->Create(This, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayModeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayModeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayModeInfo[] = L"Windows.Devices.Display.Core.IDisplayModeInfo";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePixelFormat)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_TargetResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_PresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInterlaced)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetWireFormatSupportedBitsPerChannel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding encoding,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayBitsPerChannel* result);
    HRESULT (STDMETHODCALLTYPE* IsWireFormatSupported)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* wireFormat,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* This,
        __FIMapView_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfoVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_SourceResolution(This, value) \
    ((This)->lpVtbl->get_SourceResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_SourcePixelFormat(This, value) \
    ((This)->lpVtbl->get_SourcePixelFormat(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_TargetResolution(This, value) \
    ((This)->lpVtbl->get_TargetResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_PresentationRate(This, value) \
    ((This)->lpVtbl->get_PresentationRate(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_IsInterlaced(This, value) \
    ((This)->lpVtbl->get_IsInterlaced(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_GetWireFormatSupportedBitsPerChannel(This, encoding, result) \
    ((This)->lpVtbl->GetWireFormatSupportedBitsPerChannel(This, encoding, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_IsWireFormatSupported(This, wireFormat, result) \
    ((This)->lpVtbl->IsWireFormatSupported(This, wireFormat, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayModeInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayModeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayModeInfo2[] = L"Windows.Devices.Display.Core.IDisplayModeInfo2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalPresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2* This,
        struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentationRate* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_get_PhysicalPresentationRate(This, value) \
    ((This)->lpVtbl->get_PhysicalPresentationRate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayMuxDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayMuxDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayMuxDevice[] = L"Windows.Devices.Display.Core.IDisplayMuxDevice";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetAvailableMuxTargets)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** result);
    HRESULT (STDMETHODCALLTYPE* get_CurrentTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** value);
    HRESULT (STDMETHODCALLTYPE* get_IsAutomaticTargetSwitchingEnabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetPreferredTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SetAutomaticTargetSwitching)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_GetAvailableMuxTargets(This, result) \
    ((This)->lpVtbl->GetAvailableMuxTargets(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_get_CurrentTarget(This, value) \
    ((This)->lpVtbl->get_CurrentTarget(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_get_PreferredTarget(This, value) \
    ((This)->lpVtbl->get_PreferredTarget(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_get_IsAutomaticTargetSwitchingEnabled(This, value) \
    ((This)->lpVtbl->get_IsAutomaticTargetSwitchingEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_SetPreferredTarget(This, target, operation) \
    ((This)->lpVtbl->SetPreferredTarget(This, target, operation))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_SetAutomaticTargetSwitching(This, operation) \
    ((This)->lpVtbl->SetAutomaticTargetSwitching(This, operation))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayMuxDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayMuxDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayMuxDeviceStatics[] = L"Windows.Devices.Display.Core.IDisplayMuxDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics* This,
        HSTRING deviceInterfaceId,
        __FIAsyncOperation_1_Windows__CDevices__CDisplay__CCore__CDisplayMuxDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_FromIdAsync(This, deviceInterfaceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceInterfaceId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayMuxDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPath[] = L"Windows.Devices.Display.Core.IDisplayPath";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_View)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** value);
    HRESULT (STDMETHODCALLTYPE* get_Target)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32** value);
    HRESULT (STDMETHODCALLTYPE* put_SourceResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePixelFormat)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_SourcePixelFormat)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsStereo)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TargetResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32** value);
    HRESULT (STDMETHODCALLTYPE* put_TargetResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_PresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate** value);
    HRESULT (STDMETHODCALLTYPE* put_PresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInterlaced)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_IsInterlaced)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WireFormat)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat** value);
    HRESULT (STDMETHODCALLTYPE* put_WireFormat)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_Rotation)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayRotation* value);
    HRESULT (STDMETHODCALLTYPE* put_Rotation)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayRotation value);
    HRESULT (STDMETHODCALLTYPE* get_Scaling)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathScaling* value);
    HRESULT (STDMETHODCALLTYPE* put_Scaling)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPathScaling value);
    HRESULT (STDMETHODCALLTYPE* FindModes)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayModeQueryOptions flags,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayModeInfo** result);
    HRESULT (STDMETHODCALLTYPE* ApplyPropertiesFromMode)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayModeInfo* modeResult);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* This,
        __FIMap_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPathVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_View(This, value) \
    ((This)->lpVtbl->get_View(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_Target(This, value) \
    ((This)->lpVtbl->get_Target(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_SourceResolution(This, value) \
    ((This)->lpVtbl->get_SourceResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_SourceResolution(This, value) \
    ((This)->lpVtbl->put_SourceResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_SourcePixelFormat(This, value) \
    ((This)->lpVtbl->get_SourcePixelFormat(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_SourcePixelFormat(This, value) \
    ((This)->lpVtbl->put_SourcePixelFormat(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_IsStereo(This, value) \
    ((This)->lpVtbl->put_IsStereo(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_TargetResolution(This, value) \
    ((This)->lpVtbl->get_TargetResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_TargetResolution(This, value) \
    ((This)->lpVtbl->put_TargetResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_PresentationRate(This, value) \
    ((This)->lpVtbl->get_PresentationRate(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_PresentationRate(This, value) \
    ((This)->lpVtbl->put_PresentationRate(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_IsInterlaced(This, value) \
    ((This)->lpVtbl->get_IsInterlaced(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_IsInterlaced(This, value) \
    ((This)->lpVtbl->put_IsInterlaced(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_WireFormat(This, value) \
    ((This)->lpVtbl->get_WireFormat(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_WireFormat(This, value) \
    ((This)->lpVtbl->put_WireFormat(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_Rotation(This, value) \
    ((This)->lpVtbl->get_Rotation(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_Rotation(This, value) \
    ((This)->lpVtbl->put_Rotation(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_Scaling(This, value) \
    ((This)->lpVtbl->get_Scaling(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_put_Scaling(This, value) \
    ((This)->lpVtbl->put_Scaling(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_FindModes(This, flags, result) \
    ((This)->lpVtbl->FindModes(This, flags, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_ApplyPropertiesFromMode(This, modeResult) \
    ((This)->lpVtbl->ApplyPropertiesFromMode(This, modeResult))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPath2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPath
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPath2[] = L"Windows.Devices.Display.Core.IDisplayPath2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalPresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate** value);
    HRESULT (STDMETHODCALLTYPE* put_PhysicalPresentationRate)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2* This,
        __FIReference_1_Windows__CDevices__CDisplay__CCore__CDisplayPresentationRate* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_get_PhysicalPresentationRate(This, value) \
    ((This)->lpVtbl->get_PhysicalPresentationRate(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_put_PhysicalPresentationRate(This, value) \
    ((This)->lpVtbl->put_PhysicalPresentationRate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescription[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescription";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Format)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_ColorSpace)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MultisampleDescription)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription* This,
        __FIMapView_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_Format(This, value) \
    ((This)->lpVtbl->get_Format(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_ColorSpace(This, value) \
    ((This)->lpVtbl->get_ColorSpace(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_MultisampleDescription(This, value) \
    ((This)->lpVtbl->get_MultisampleDescription(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescriptionFactory[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory* This,
        UINT32 width,
        UINT32 height,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace colorSpace,
        boolean isStereo,
        struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription multisampleDescription,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_CreateInstance(This, width, height, pixelFormat, colorSpace, isStereo, multisampleDescription, value) \
    ((This)->lpVtbl->CreateInstance(This, width, height, pixelFormat, colorSpace, isStereo, multisampleDescription, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayPrimaryDescriptionStatics[] = L"Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithProperties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics* This,
        __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* extraProperties,
        UINT32 width,
        UINT32 height,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXColorSpace colorSpace,
        boolean isStereo,
        struct __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CDirect3DMultisampleDescription multisampleDescription,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescription** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_CreateWithProperties(This, extraProperties, width, height, pixelFormat, colorSpace, isStereo, multisampleDescription, result) \
    ((This)->lpVtbl->CreateWithProperties(This, extraProperties, width, height, pixelFormat, colorSpace, isStereo, multisampleDescription, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPrimaryDescriptionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayScanout
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayScanout
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayScanout[] = L"Windows.Devices.Display.Core.IDisplayScanout";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanoutVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanoutVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanoutVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySource[] = L"Windows.Devices.Display.Core.IDisplaySource";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdapterId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetMetadata)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource* This,
        GUID Key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySourceVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_get_AdapterId(This, value) \
    ((This)->lpVtbl->get_AdapterId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_get_SourceId(This, value) \
    ((This)->lpVtbl->get_SourceId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_GetMetadata(This, Key, result) \
    ((This)->lpVtbl->GetMetadata(This, Key, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySource2[] = L"Windows.Devices.Display.Core.IDisplaySource2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplaySourceStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        __FITypedEventHandler_2_Windows__CDevices__CDisplay__CCore__CDisplaySource_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayState[] = L"Windows.Devices.Display.Core.IDisplayState";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStale)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Targets)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayTarget** value);
    HRESULT (STDMETHODCALLTYPE* get_Views)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayView** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __FIMap_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* ConnectTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** result);
    HRESULT (STDMETHODCALLTYPE* ConnectTargetToView)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* view,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** result);
    HRESULT (STDMETHODCALLTYPE* CanConnectTargetToView)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* view,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetViewForTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView** result);
    HRESULT (STDMETHODCALLTYPE* GetPathForTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath** result);
    HRESULT (STDMETHODCALLTYPE* DisconnectTarget)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* target);
    HRESULT (STDMETHODCALLTYPE* TryFunctionalize)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateFunctionalizeOptions options,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult** result);
    HRESULT (STDMETHODCALLTYPE* TryApply)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateApplyOptions options,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult** result);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_get_IsStale(This, value) \
    ((This)->lpVtbl->get_IsStale(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_get_Targets(This, value) \
    ((This)->lpVtbl->get_Targets(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_get_Views(This, value) \
    ((This)->lpVtbl->get_Views(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_ConnectTarget(This, target, result) \
    ((This)->lpVtbl->ConnectTarget(This, target, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_ConnectTargetToView(This, target, view, result) \
    ((This)->lpVtbl->ConnectTargetToView(This, target, view, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_CanConnectTargetToView(This, target, view, result) \
    ((This)->lpVtbl->CanConnectTargetToView(This, target, view, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_GetViewForTarget(This, target, result) \
    ((This)->lpVtbl->GetViewForTarget(This, target, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_GetPathForTarget(This, target, result) \
    ((This)->lpVtbl->GetPathForTarget(This, target, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_DisconnectTarget(This, target) \
    ((This)->lpVtbl->DisconnectTarget(This, target))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_TryFunctionalize(This, options, result) \
    ((This)->lpVtbl->TryFunctionalize(This, options, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_TryApply(This, options, result) \
    ((This)->lpVtbl->TryApply(This, options, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_Clone(This, result) \
    ((This)->lpVtbl->Clone(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayStateOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayStateOperationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayStateOperationResult[] = L"Windows.Devices.Display.Core.IDisplayStateOperationResult";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayStateOperationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedErrorCode)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResultVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_get_ExtendedErrorCode(This, value) \
    ((This)->lpVtbl->get_ExtendedErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayStateOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplaySurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplaySurface
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplaySurface[] = L"Windows.Devices.Display.Core.IDisplaySurface";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurfaceVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplaySurface_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTarget[] = L"Windows.Devices.Display.Core.IDisplayTarget";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Adapter)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayAdapter** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInterfacePath)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AdapterRelativeId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnected)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVirtualModeEnabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVirtualTopologyEnabled)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageKind)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CDisplayMonitorUsageKind* value);
    HRESULT (STDMETHODCALLTYPE* get_MonitorPersistence)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTargetPersistence* value);
    HRESULT (STDMETHODCALLTYPE* get_StableMonitorId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* TryGetMonitor)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CIDisplayMonitor** result);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        __FIMapView_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_IsStale)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsSame)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* otherTarget,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsEqual)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget* otherTarget,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTargetVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_Adapter(This, value) \
    ((This)->lpVtbl->get_Adapter(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_DeviceInterfacePath(This, value) \
    ((This)->lpVtbl->get_DeviceInterfacePath(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_AdapterRelativeId(This, value) \
    ((This)->lpVtbl->get_AdapterRelativeId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_IsConnected(This, value) \
    ((This)->lpVtbl->get_IsConnected(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_IsVirtualModeEnabled(This, value) \
    ((This)->lpVtbl->get_IsVirtualModeEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_IsVirtualTopologyEnabled(This, value) \
    ((This)->lpVtbl->get_IsVirtualTopologyEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_UsageKind(This, value) \
    ((This)->lpVtbl->get_UsageKind(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_MonitorPersistence(This, value) \
    ((This)->lpVtbl->get_MonitorPersistence(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_StableMonitorId(This, value) \
    ((This)->lpVtbl->get_StableMonitorId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_TryGetMonitor(This, result) \
    ((This)->lpVtbl->TryGetMonitor(This, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_get_IsStale(This, value) \
    ((This)->lpVtbl->get_IsStale(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_IsSame(This, otherTarget, result) \
    ((This)->lpVtbl->IsSame(This, otherTarget, result))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_IsEqual(This, otherTarget, result) \
    ((This)->lpVtbl->IsEqual(This, otherTarget, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTask[] = L"Windows.Devices.Display.Core.IDisplayTask";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetScanout)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayScanout* scanout);
    HRESULT (STDMETHODCALLTYPE* SetWait)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* readyFence,
        UINT64 readyFenceValue);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_SetScanout(This, scanout) \
    ((This)->lpVtbl->SetScanout(This, scanout))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_SetWait(This, readyFence, readyFenceValue) \
    ((This)->lpVtbl->SetWait(This, readyFence, readyFenceValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTask2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTask2[] = L"Windows.Devices.Display.Core.IDisplayTask2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetSignal)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayTaskSignalKind signalKind,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayFence* fence);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_SetSignal(This, signalKind, fence) \
    ((This)->lpVtbl->SetSignal(This, signalKind, fence))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskPool[] = L"Windows.Devices.Display.Core.IDisplayTaskPool";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPoolVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTask)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DEPRECATED("Use TryExecuteTask instead of ExecuteTask. For more info see MSDN")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    HRESULT (STDMETHODCALLTYPE* ExecuteTask)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* task);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPoolVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPoolVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_CreateTask(This, result) \
    ((This)->lpVtbl->CreateTask(This, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DEPRECATED("Use TryExecuteTask instead of ExecuteTask. For more info see MSDN")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_ExecuteTask(This, task) \
    ((This)->lpVtbl->ExecuteTask(This, task))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskPool2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskPool2[] = L"Windows.Devices.Display.Core.IDisplayTaskPool2";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryExecuteTask)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTask* task,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2Vtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_TryExecuteTask(This, task, result) \
    ((This)->lpVtbl->TryExecuteTask(This, task, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskPool2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayTaskResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayTaskResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayTaskResult[] = L"Windows.Devices.Display.Core.IDisplayTaskResult";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PresentStatus)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayPresentStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_PresentId)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceStatus)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplaySourceStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResultVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_get_PresentStatus(This, value) \
    ((This)->lpVtbl->get_PresentStatus(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_get_PresentId(This, value) \
    ((This)->lpVtbl->get_PresentId(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_get_SourceStatus(This, value) \
    ((This)->lpVtbl->get_SourceStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayTaskResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayView[] = L"Windows.Devices.Display.Core.IDisplayView";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Paths)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        __FIVectorView_1_Windows__CDevices__CDisplay__CCore__CDisplayPath** value);
    HRESULT (STDMETHODCALLTYPE* get_ContentResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32** value);
    HRESULT (STDMETHODCALLTYPE* put_ContentResolution)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        __FIReference_1_Windows__CGraphics__CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* SetPrimaryPath)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayPath* path);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView* This,
        __FIMap_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayViewVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_get_Paths(This, value) \
    ((This)->lpVtbl->get_Paths(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_get_ContentResolution(This, value) \
    ((This)->lpVtbl->get_ContentResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_put_ContentResolution(This, value) \
    ((This)->lpVtbl->put_ContentResolution(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_SetPrimaryPath(This, path) \
    ((This)->lpVtbl->SetPrimaryPath(This, path))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormat[] = L"Windows.Devices.Display.Core.IDisplayWireFormat";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelEncoding)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding* value);
    HRESULT (STDMETHODCALLTYPE* get_BitsPerChannel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ColorSpace)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace* value);
    HRESULT (STDMETHODCALLTYPE* get_Eotf)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf* value);
    HRESULT (STDMETHODCALLTYPE* get_HdrMetadata)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat* This,
        __FIMapView_2_GUID_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_PixelEncoding(This, value) \
    ((This)->lpVtbl->get_PixelEncoding(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_BitsPerChannel(This, value) \
    ((This)->lpVtbl->get_BitsPerChannel(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_ColorSpace(This, value) \
    ((This)->lpVtbl->get_ColorSpace(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_Eotf(This, value) \
    ((This)->lpVtbl->get_Eotf(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_HdrMetadata(This, value) \
    ((This)->lpVtbl->get_HdrMetadata(This, value))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormatFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormatFactory[] = L"Windows.Devices.Display.Core.IDisplayWireFormatFactory";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory* This,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding pixelEncoding,
        INT32 bitsPerChannel,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace colorSpace,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf eotf,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata hdrMetadata,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_CreateInstance(This, pixelEncoding, bitsPerChannel, colorSpace, eotf, hdrMetadata, value) \
    ((This)->lpVtbl->CreateInstance(This, pixelEncoding, bitsPerChannel, colorSpace, eotf, hdrMetadata, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Display.Core.IDisplayWireFormatStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Display.Core.DisplayWireFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Display_Core_IDisplayWireFormatStatics[] = L"Windows.Devices.Display.Core.IDisplayWireFormatStatics";
typedef struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithProperties)(__x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics* This,
        __FIIterable_1___FIKeyValuePair_2_GUID_IInspectable* extraProperties,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatPixelEncoding pixelEncoding,
        INT32 bitsPerChannel,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatColorSpace colorSpace,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatEotf eotf,
        enum __x_ABI_CWindows_CDevices_CDisplay_CCore_CDisplayWireFormatHdrMetadata hdrMetadata,
        __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormat** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_CreateWithProperties(This, extraProperties, pixelEncoding, bitsPerChannel, colorSpace, eotf, hdrMetadata, result) \
    ((This)->lpVtbl->CreateWithProperties(This, extraProperties, pixelEncoding, bitsPerChannel, colorSpace, eotf, hdrMetadata, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CDisplay_CCore_CIDisplayWireFormatStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayAdapter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayAdapterStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayAdapter ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayAdapter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayAdapter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayAdapter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayAdapter[] = L"Windows.Devices.Display.Core.DisplayAdapter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayDevice ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayDevice2
 *    Windows.Devices.Display.Core.IDisplayDeviceRenderAdapter
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayDevice[] = L"Windows.Devices.Display.Core.DisplayDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayFence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayFence ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayFence_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayFence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayFence[] = L"Windows.Devices.Display.Core.DisplayFence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayManagerStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManager ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayManager2
 *    Windows.Devices.Display.Core.IDisplayManager3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManager_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManager[] = L"Windows.Devices.Display.Core.DisplayManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerChangedEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerDisabledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerDisabledEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerEnabledEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerEnabledEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerPathsFailedOrInvalidatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerPathsFailedOrInvalidatedEventArgs[] = L"Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayManagerResultWithState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayManagerResultWithState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerResultWithState_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayManagerResultWithState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayManagerResultWithState[] = L"Windows.Devices.Display.Core.DisplayManagerResultWithState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayModeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayModeInfo ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayModeInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayModeInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayModeInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayModeInfo[] = L"Windows.Devices.Display.Core.DisplayModeInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayMuxDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayMuxDeviceStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayMuxDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayMuxDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayMuxDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayMuxDevice[] = L"Windows.Devices.Display.Core.DisplayMuxDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayPath ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayPath2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPath_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPath_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayPath[] = L"Windows.Devices.Display.Core.DisplayPath";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayPrimaryDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Display.Core.IDisplayPrimaryDescriptionFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayPrimaryDescriptionStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayPrimaryDescription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPrimaryDescription_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayPrimaryDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayPrimaryDescription[] = L"Windows.Devices.Display.Core.DisplayPrimaryDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayScanout
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayScanout ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayScanout_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayScanout_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayScanout[] = L"Windows.Devices.Display.Core.DisplayScanout";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplaySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplaySource ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplaySource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySource_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplaySource[] = L"Windows.Devices.Display.Core.DisplaySource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayState_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayState[] = L"Windows.Devices.Display.Core.DisplayState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayStateOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayStateOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayStateOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayStateOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayStateOperationResult[] = L"Windows.Devices.Display.Core.DisplayStateOperationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplaySurface
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplaySurface ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySurface_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplaySurface_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplaySurface[] = L"Windows.Devices.Display.Core.DisplaySurface";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTarget ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTarget_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTarget[] = L"Windows.Devices.Display.Core.DisplayTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTask ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayTask2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTask_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTask[] = L"Windows.Devices.Display.Core.DisplayTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTaskPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTaskPool ** Default Interface **
 *    Windows.Devices.Display.Core.IDisplayTaskPool2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskPool_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskPool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTaskPool[] = L"Windows.Devices.Display.Core.DisplayTaskPool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayTaskResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayTaskResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayTaskResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayTaskResult[] = L"Windows.Devices.Display.Core.DisplayTaskResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayView ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayView_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayView[] = L"Windows.Devices.Display.Core.DisplayView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Display.Core.DisplayWireFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Display.Core.IDisplayWireFormatFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Display.Core.IDisplayWireFormatStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Display.Core.IDisplayWireFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Display_Core_DisplayWireFormat_DEFINED
#define RUNTIMECLASS_Windows_Devices_Display_Core_DisplayWireFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Display_Core_DisplayWireFormat[] = L"Windows.Devices.Display.Core.DisplayWireFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Edisplay2Ecore_p_h__

#endif // __windows2Edevices2Edisplay2Ecore_h__
